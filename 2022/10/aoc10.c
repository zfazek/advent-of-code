#include "../../lib/utils_file.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 10
#define CRT_SIZE 40

int cs[] = {20, 60, 100, 140, 180, 220};
char crt[CRT_SIZE];

int cs_contains(const int v) {
    for (int i = 0; i < sizeof(cs) / sizeof(int); ++i) {
        if (cs[i] == v) {
            return 1;
        }
    }
    return 0;
}

void foo(int n, int sum, long *ans) {
    int pos = (n % CRT_SIZE) - 1;
    if (pos >= sum - 1 && pos <= sum + 1) {
        crt[pos] = '#';
    }
    if (cs_contains(n)) {
        *ans += n * sum;
    }
    if (n % CRT_SIZE == 0) {
        printf("%s\n", crt);
        memset(crt, '.', CRT_SIZE);
    }
}

void one(char lines[][BUFFER_SIZE], const int n_lines) {
    int sum = 1;
    long ans = 0;
    int n = 1;
    for (int i = 0; i < n_lines; ++i) {
        char c = lines[i][0];
        int v = atoi(lines[i] + 5);
        foo(n, sum, &ans);
        if (c == 'n') {
            ++n;
        } else {
            ++n;
            foo(n, sum, &ans);
            ++n;
            sum += v;
        }
    }
    printf("%ld\n", ans);
}

int main() {
    char fname[] = "10.txt";
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
    fclose(f);
    memset(crt, '.', CRT_SIZE);
    one(lines, n_lines);
}
