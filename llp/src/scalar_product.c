#include <stdio.h>

int one[] = { 1, 2, 3 };
int two[] = { 4, 5, 6 };

int product(int x, int y) {
  return x * y;
}

int main(void) {
  int len_one = sizeof(one) / sizeof(int);
  int len_two = sizeof(two) / sizeof(int);
  int accumulator = 0;
  int i;

  if (len_one != len_two) {
    puts("Arrays must be the same length");
    return 1;
  }

  for (i = 0; i < len_one; i++) {
    accumulator += product(one[i], two[i]);
  }

  printf("%d\n", accumulator);

  return 0;
}

