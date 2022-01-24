#include "../../lib/utils_ds.h"
#include "../../lib/utils_file.h"
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 50002

// dabAcCaCBAcCcaDA

void one(char *input) {
    const int len = strlen(input);
    const int dist = 'a' - 'A';
    char *stack = calloc(len + 1, sizeof(char));
    int idx = 0;
    for (int i = 0; i < len; ++i) {
        const char c = input[i];
        if (i == 0 || idx == 0) {
            stack[idx++] = c;
            stack[idx] = 0;
            continue;
        }
        const char top = stack[idx - 1];
        if (abs(c - top) == dist) {
            --idx;
            stack[idx] = 0;
        } else {
            stack[idx++] = c;
            stack[idx] = 0;
        }
    }
    printf("%lu\n", strlen(stack));
    free(stack);
}

void two(char *input) {
    const int len = strlen(input);
    int min = len;
    const int dist = 'a' - 'A';
    for (char c = 'A'; c <= 'Z'; ++c) {
        char *stack = calloc(len + 1, sizeof(char));
        int idx = 0;
        for (int i = 0; i < len; ++i) {
            const char ch = input[i];
            if (ch == c || ch == c + dist) {
                continue;
            }
            if (i == 0 || idx == 0) {
                stack[idx++] = ch;
                stack[idx] = 0;
                continue;
            }
            const char top = stack[idx - 1];
            if (abs(ch - top) == dist) {
                --idx;
                stack[idx] = 0;
            } else {
                stack[idx++] = ch;
                stack[idx] = 0;
            }
        }
        if (strlen(stack) < min) {
            min = strlen(stack);
        }
        free(stack);
    }
    printf("%d\n", min);
}

int main() {
    char fname[] = "05.txt";
    FILE *f = fopen(fname, "r");
    if (f == NULL) {
        exit(1);
    }
    char buffer[BUFFER_SIZE];
    int idx = 0;
    char *c = fgets(buffer, BUFFER_SIZE, f);
    fclose(f);
    buffer[strcspn(buffer, "\n")] = 0;
    one(buffer);
    two(buffer);
}
