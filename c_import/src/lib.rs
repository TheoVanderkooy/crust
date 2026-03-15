use proc_macro::TokenStream;
use syn::parse_macro_input;

use quote::{ToTokens, quote};

/// Macro used to wrap C functions that may "throw" by longjmp
///
/// For example: if C has the following function
/// ```C
/// extern int foo(int x, bool y);
/// ```
///
/// The function should be imported using the macro as follows:
/// ```rs
/// #[c_import(foo)]
/// fn bar(x: c_int, y: bool) -> c_int;
/// ```
///
/// And then `bar` will be exposed as an unsafe function in rust, with signature `(c_int, bool) -> Result<c_int, PgError>`
#[proc_macro_attribute]
pub fn c_import(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::ForeignItemFn);

    // Name of the C function can be provided, or use the existing one
    let c_fn_name = if attr.is_empty() {
        ast.sig.ident.clone()
    } else {
        parse_macro_input!(attr as syn::Ident)
    };

    println!("new fn name: {c_fn_name:#?}");

    let mut c_sig = ast.sig.clone();
    let new_fn_name = c_sig.ident.clone();
    c_sig.ident = c_fn_name.clone();

    let attrs = ast.attrs;
    let vis = ast.vis;

    let inputs = c_sig.inputs.clone();
    let ret_t = match c_sig.output.clone() {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, t) => t.to_token_stream(),
    };

    let args: Vec<_> = inputs
        .iter()
        .filter_map(|a| match a {
            syn::FnArg::Typed(pt) => Some(pt.pat.clone()),
            syn::FnArg::Receiver(_) => None,
        })
        .collect();

    let ret = quote!(

        #(#attrs)*
        #vis unsafe fn #new_fn_name(#inputs) -> Result<#ret_t, PgError> {

            unsafe extern "C-unwind" {
                #[doc(hidden)]
                unsafe #c_sig ;
            }

            unsafe {
                let save_stack = PG_exception_stack;
                let mut local_jmp_buf: sigjmp_buf = MaybeUninit::zeroed().assume_init();
                let ret = if __sigsetjmp(local_jmp_buf.as_mut_ptr(), 1) == 0 {
                    PG_exception_stack = &mut local_jmp_buf;

                    Ok(#c_fn_name (#(#args,)*))

                } else {
                    PG_exception_stack = save_stack;

                    Err(PgError)
                };
                PG_exception_stack = save_stack;

                ret
            }

        }
    )
    .into();

    println!("PRODUCED RESULT:\n {ret}");

    ret
}

// TODO: #[C_import_infallible]
//  - like c_import, but returns value directly instead of result
//  - still does sigsetjmp, but if it catches something it panics instead

// https://ferrous-systems.com/blog/testing-proc-macros/

/// Macro used to wrap rust functions and export them to C.
/// This converts Result<T, PgError> to either returning T or propagating the error as an ereport.
/// If the rust code panics, converts to an ereport(PANIC).
///
/// Example:
/// ```rs
/// @[rust_export(foo)]
/// pub fn bar(x: c_int) -> Result<c_int, PgError> {
///   Ok(2 * x)
/// }
/// ```
///
/// This will expose the following function that can be used in C:
/// ```c
/// extern int foo(int);
/// ```
/// Note that you'll have to add it to a header file manually
#[proc_macro_attribute]
pub fn rust_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Input should be a rust function
    let ast = parse_macro_input!(item as syn::ItemFn);

    let fn_name = ast.sig.ident.clone();
    let c_fn_name = parse_macro_input!(attr as syn::Ident);

    let inputs = ast.sig.inputs.clone();
    let output = ast.sig.output.clone();

    let input_args: Vec<_> = inputs
        .clone()
        .iter()
        .filter_map(|a| match a {
            syn::FnArg::Typed(pt) => Some(pt.pat.clone()),
            syn::FnArg::Receiver(_) => None,
        })
        .collect();

    let output = if let syn::ReturnType::Type(_, bt) = output
        && let syn::Type::Path(tp) = *bt
        && let Some(last) = tp.path.segments.last()
        && last.ident == syn::Ident::new("Result", last.ident.span().clone())
        && let syn::PathArguments::AngleBracketed(generics) = &last.arguments
        && let Some(t) = generics.args.first()
    {
        t.to_token_stream()
    } else {
        return quote!(compile_error!(
            "unexpected return type, expected Result<T, PgError>"
        ))
        .into();
    };

    let ret = quote! {
        #ast

        #[unsafe(no_mangle)]
        pub extern "C" fn #c_fn_name( #inputs ) -> #output {
            // TODO: should also catch rust panics and convert to ereport panic
            let res = #fn_name (#(#input_args,)*) ;
            match res {
                Ok(r) => r,
                Err(_) => todo!(), // TODO: should longjmp here
            }
        }
    }
    .into();

    println!("RESULT =\n{ret}");

    ret
}
