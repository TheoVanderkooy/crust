
#include"../include/foo.h"
#include<stdio.h>


int test(char* str) {
  printf("message : %s\n", str);
  return 5;
}

const char * test_str(int x) {
  return "borring message from c";
}