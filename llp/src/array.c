#include <stdio.h>

int first (int array[], size_t sz) {
  if (sz == 0) {
    return -1;
  }

  return array[0];
}

int ptr_first (int* array, size_t sz) {
  if (sz == 0) {
    return -1;
  }

  return *array;
}

int mix_first (int* array, size_t sz) {
  if (sz == 0) {
    return -1;
  }

  return array[0];
}

int main(void) {
  int sz = 5;
  int array[5] = { 1, 2, 3, 4, 5 };

  printf("%d\n", first(array, sz));
  printf("%d\n", ptr_first(array, sz));
  printf("%d\n", mix_first(array, sz));

  return 0;
}
