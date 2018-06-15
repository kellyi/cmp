#include <stdio.h>

double g(int n) {
  return 0.5 + n;
}

double apply(double (*f)(int), int x) {
  return f(x);
}

int main(void) {
  printf("%f\n", apply(g, 10));
  return 0;
}
