#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "../../lib/utils_file.h"

#define BUFFER_SIZE 200

void one(char lines[][BUFFER_SIZE], int n_lines) {
    long sum = 0;
    for (int i = 0; i < n_lines; ++i) {
        sum += atoi(lines[i]) / 3 - 2;
    }
    printf("%ld\n", sum);
}

void two(char lines[][BUFFER_SIZE], int n_lines) {
    long sum = 0;
    for (int i = 0; i < n_lines; ++i) {
        long s = 0;
        int n = atoi(lines[i]);
        while (n > 0) {
            n = n / 3 - 2;
            if (n > 0) {
                s += n;
            }
        }
        sum += s;
    }
    printf("%ld\n", sum);
}

int main() {
    char fname[] = "01.txt";
    FILE *f = fopen(fname, "r");
    if (f == NULL) {
        exit(1);
    }
    int n_lines = get_number_of_lines(f);
    fseek(f, 0, SEEK_SET);
    char lines[n_lines][BUFFER_SIZE];
    char buffer[BUFFER_SIZE];
    int idx = 0;
    while (fgets(buffer, BUFFER_SIZE, f)) {
        strcpy(lines[idx++], buffer);
    }
    one(lines, n_lines);
    two(lines, n_lines);
}
