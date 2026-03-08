#include<stdio.h>
#include "include/foo.h"

int main(){
  int x = test("abc");
  printf("ret : %d\n", x);

  const char* y = test_str(432);
  printf("string ret : %s\n", y);
}
