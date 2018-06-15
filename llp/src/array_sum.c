#include <stdio.h>
int array[] = { 1, 2, 3, 4, 5 };

int sum_list(int l[], size_t count) {
  int i;
  int accumulator = 0;

  for (i = 0; i < count; i++) {
    accumulator += l[i];
  }

  return accumulator;
}

int main(int argc, char** argv) {
  const size_t count = sizeof(array) / sizeof(int);
  const int sum = sum_list(array, count);

  printf("sum is %d\n", sum);

  return 0;
}
