
// bindgen src/test.h > src/bindings.rs


#include<setjmp.h>
#include<stdbool.h>

extern sigjmp_buf *PG_exception_stack;

extern void throws();

extern int maybe_throws(int x, bool dothrow);