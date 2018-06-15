#include <stdio.h>

int main(void) {
  int i;

  i = 0;

  while (i < 10) {
    puts("hello");

    i += 1;
  }

  for (i = 0; i < 10; i++) {
    puts("Hello");
  }

  return i;
}
