#include <malloc.h>

int main(void) {
  int* array;

  array = malloc(10 * sizeof(array));

  free(array);

  return 0;
}
