#include <stdio.h>

int puts(const char* str) {
    return printf("Intercepted `puts` call and replaced it!");
}

