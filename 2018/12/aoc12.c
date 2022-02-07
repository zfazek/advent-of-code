#include "../../lib/utils_file.h"
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 500
#define N 100

// initial state:
// ##.#.#.##..#....######..#..#...#.#..#.#.#..###.#.#.#..#..###.##.#..#.##.##.#.####..##...##..#..##.#.
//
// ...## => #

typedef struct Rule {
    char str[6];
    char result;
} Rule;

char *arr1;
char *arr2;
Rule *rules;

void read_input(char lines[][BUFFER_SIZE], int n_lines) {
    rules = malloc((n_lines - 2) * sizeof(Rule));
    for (int i = 0; i < n_lines; ++i) {
        char *line = lines[i];
        if (i == 0) {
            line = line + 15;
            int size = strlen(line) - 1;
            arr1 = calloc(size + N * 4 + 1, sizeof(char));
            arr2 = calloc(size + N * 4 + 1, sizeof(char));
            for (int j = 0; j < size + N * 4; ++j) {
                arr1[j] = '.';
            }
            memcpy(arr1 + N * 2, line, size);
            arr1[size + N * 4] = 0;
            ++i;
        } else {
            int idx = i - 2;
            Rule rule;
            strncpy(rule.str, line, 5);
            rule.str[5] = 0;
            rule.result = line[9];
            rules[idx] = rule;
        }
    }
}

void foo(char lines[][BUFFER_SIZE], int n_lines) {
    long n = 50000000000;
    long n1 = 100;
    read_input(lines, n_lines);
    for (int i = 1; i <= n1; ++i) {
        strcpy(arr2, arr1);
        for (unsigned j = 0; j < strlen(arr1) - 5; ++j) {
            arr2[j + 2] = '.';
            for (int k = 0; k < n_lines; ++k) {
                if (strncmp(arr1 + j, rules[k].str, 5) == 0) {
                    arr2[j + 2] = rules[k].result;
                    break;
                }
            }
        }
        strcpy(arr1, arr2);
        if (i == 20) {
            int sum = 0;
            for (unsigned i = 0; i < strlen(arr1); ++i) {
                if (arr1[i] == '#') {
                    sum += i - 2 * N;
                }
            }
            printf("%d\n", sum);
        }
    }
    unsigned long sum = 0;
    for (unsigned i = 0; i < strlen(arr1); ++i) {
        if (arr1[i] == '#') {
            sum += i - 2 * N + n - n1;
        }
    }
    printf("%lu\n", sum);
}

int main() {
    char fname[] = "12.txt";
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
    foo(lines, n_lines);
}
