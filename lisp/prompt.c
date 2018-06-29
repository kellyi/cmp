#include <stdio.h>
#include <stdlib.h>
#include <editline/readline.h>

int main(int argc, char** argv) {
    puts("Lispy version 0.0.0.0.1");
    puts("Press Ctrl+C to exit\n");

    while (1) {
        char* input = readline("λ> ");
        add_history(input);
        printf("Repeating: %s\n", input);

        free(input);
    }

    return 0;
}

