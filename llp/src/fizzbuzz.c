#include <stdio.h>
#define IS_BUZZ(x) (x % 5 == 0)
#define IS_FIZZ(x) (x % 3 == 0)
#define IS_NEITHER(x) ((x % 5 != 0) && (x % 3 != 0))

void buzz() {
  printf("buzz");
}

void fizz() {
  printf("fizz");
}

void newline() {
  printf("\n");
}

void number(int n) {
  printf("%d", n);
}

int main(void) {
  int x;

  for (x = 1; x <= 100; x++) {
    if (IS_NEITHER(x)) {
      number(x);
      goto next;
    }

    if (IS_FIZZ(x)) {
      fizz();
    }

    if (IS_BUZZ(x)) {
      buzz();
    }

    next: newline();
  }

  return 0;
}
