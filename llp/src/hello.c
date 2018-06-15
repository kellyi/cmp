/* This is a comment; the next line has a pre-processor directive */
#include <stdio.h>

/* `main` is the entry point for the program, equivalent to `_start` in ASM
 * `_start` calls main; main returns its return code to the exit call */

int main(void)
{
  int x = 43;
  int y;
  y = x - 1;

  printf("Hello, world y=%d, x=%d\n", y, x);

  return 0;
}
