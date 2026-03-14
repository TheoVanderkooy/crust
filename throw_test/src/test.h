
#include<setjmp.h>

extern sigjmp_buf *PG_exception_stack;

extern void throws();