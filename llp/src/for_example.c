#include <stdio.h>

int main(void) {
  int a[] = { 1, 2, 3, 4 };

  int i;

  for (i = 0; i < 4; i++) {
    printf("at index %d -> %d\n", i, a[i]);
  }

  return i;
}
