#include <stdio.h>

int main(void) {
  int i;
  for (i = 0; ; i++) {
    printf("%d\n", i);

    if (i == 10) {
      goto end;
    }
  }

  end: return 0;
}


