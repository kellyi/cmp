#include <stdio.h>

int main(int argc, char** argv) {
    int ordinary = 0;
    volatile int vol = 1;
    ordinary++;
    vol++;
    printf("%d\n", ordinary);
    printf("%d\n", vol);

    return 0;
}

