#include <stdio.h>

int main(void) {
  int i = 1;

  start:
  switch (i) {
    case 1:
      puts("one");
      i += 1;
      goto start;
    case 2:
      puts("two");
      i += 1;
      goto start;
    case 3:
      puts("three");
    default:
      goto end;
  }

  end: return i;
}
