
// #![no_std]

// #![feature(allocator_api)]

// struct thingy;
// use std::alloc::{Allocator, GlobalAlloc};

// unsafe impl Allocator for thingy {
//     fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
//         todo!()
//     }

//     unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
//         todo!()
//     }
// }

// impl thingy {
//     const fn new() -> Self {
//         thingy{}
//     }
// }

// unsafe impl GlobalAlloc for thingy {
//     unsafe fn alloc(&self, _layout: std::alloc::Layout) -> *mut u8 {
//         todo!()
//     }

//     unsafe fn dealloc(&self, _ptr: *mut u8, _layout: std::alloc::Layout) {
//         todo!()
//     }
// }


// #[global_allocator]
// static THINGY: thingy = thingy::new();


mod bindings;
pub use bindings::test;