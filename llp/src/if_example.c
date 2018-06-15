#include <stdio.h>


int main(void) {
  int x = 100;

  if (42) {
    puts("42 is truthy because non-zero");
  }

  if (x > 3) {
    printf("%d is greater than 3\n", x);
  } else {
    printf("%d is not greater than 3\n", x);
  }

  return 0;
}
