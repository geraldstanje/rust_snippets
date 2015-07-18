#include <iostream>
#include "add.h"
#include "hello.h"

int main(int argc, char *argv[]) {
  int res = add(2, 3);
  std::cout << res << std::endl;

  hello("hello world");
}