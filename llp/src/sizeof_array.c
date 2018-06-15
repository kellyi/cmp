#include <stdio.h>

long array[] = { 1, 2, 3 };

int main(void) {
  printf("%lu\n", sizeof(array));
  printf("%lu\n", sizeof(*array));

  return 0;
}
