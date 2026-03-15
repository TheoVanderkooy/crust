

/*
// gcc -c test.c -o test.o


*/

#include<stdio.h>
#include<setjmp.h>
#include<stdbool.h>
#include "test.h"

sigjmp_buf *PG_exception_stack = NULL;

#define PG_TRY(...)  \
	do { \
		sigjmp_buf *_save_exception_stack##__VA_ARGS__ = PG_exception_stack; \
		sigjmp_buf _local_sigjmp_buf##__VA_ARGS__; \
		if (sigsetjmp(_local_sigjmp_buf##__VA_ARGS__, 0) == 0) \
		{ \
			PG_exception_stack = &_local_sigjmp_buf##__VA_ARGS__ ;

#define PG_CATCH(...)	\
		} \
		else \
		{ \
			PG_exception_stack = _save_exception_stack##__VA_ARGS__; \

#define PG_END_TRY(...)  \
		} \
		PG_exception_stack = _save_exception_stack##__VA_ARGS__; \
	} while (0)

void throw(void) {
    if (PG_exception_stack != NULL) {
        siglongjmp(*PG_exception_stack, 1);
    } else {
        printf("Not in an exception path!!\n");
    }
}

void throws()
{
    printf("pre-throw\n");
    // printf("jmp buf: %x", PG_exception_stack);
    throw();
    printf("after throw (uncreachable)\n");
}




int maybe_throws(int x, bool dothrow)
{
    if (dothrow) {
        throw();
    }
    return 2 * x;
}



void TEST_throw_main() {
    printf("test\n");
    PG_TRY()
    {
        throws();
        printf("didn't throw\n");
    }
    PG_CATCH()
    {
        printf("caught exception\n");
    }
    PG_END_TRY();


}

// int main() { TEST_throw_main(); }