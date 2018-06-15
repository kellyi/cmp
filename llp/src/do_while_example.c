#include <stdio.h>

int main(void) {
  int x = 10;

  do {
    printf("%d\n", x);
    x -= 1;
  } while (x > 0);

  return x;
}
