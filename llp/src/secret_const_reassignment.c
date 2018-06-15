#include <stdio.h>

int main(void) {
  const int x = 10;
  printf("%d\n", x);

  *( (int*)&x ) = 30;
  printf("%d\n", x);

  return 0;
}
