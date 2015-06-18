#include <stdio.h>
#include <stdlib.h>
#include "add.h"
#include "hello.h"

int main(int argc, char *argv[]) {
  int res = add(2, 3);

  printf("%d", res);

  hello("hello world");
}