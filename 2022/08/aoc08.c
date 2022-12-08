#include "../../lib/utils_file.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 101

int is_visible(char lines[][BUFFER_SIZE], const int n, const int x, const int y) {
    int found = 0;
    for (int i = 0; i < x; ++i) {
        if (lines[i][y] >= lines[x][y]) {
            found = 1;
            break;
        }
    }
    if (!found) {
        return 1;
    }
    found = 0;
    for (int i = x + 1; i < n; ++i) {
        if (lines[i][y] >= lines[x][y]) {
            found = 1;
            break;
        }
    }
    if (!found) {
        return 1;
    }
    found = 0;
    for (int i = 0; i < y; ++i) {
        if (lines[x][i] >= lines[x][y]) {
            found = 1;
            break;
        }
    }
    if (!found) {
        return 1;
    }
    found = 0;
    for (int i = y + 1; i < n; ++i) {
        if (lines[x][i] >= lines[x][y]) {
            found = 1;
            break;
        }
    }
    if (!found) {
        return 1;
    }
    return 0;
}

void one(char lines[][BUFFER_SIZE], const int n_lines) {
    int sum = 0;
    for (int i = 0; i < n_lines; ++i) {
        for (int j = 0; j < n_lines; ++j) {
            sum += is_visible(lines, n_lines, i, j);
        }
    }
    printf("%d\n", sum);
}

int get_num_trees(char lines[][BUFFER_SIZE], const int n, const int x, const int y) {
    int sum1 = 0;
    for (int i = x - 1; i >= 0; --i) {
        ++sum1;
        if (lines[i][y] >= lines[x][y]) {
            break;
        }
    }
    int sum2 = 0;
    for (int i = x + 1; i < n; ++i) {
        ++sum2;
        if (lines[i][y] >= lines[x][y]) {
            break;
        }
    }
    int sum3 = 0;
    for (int i = y - 1; i >= 0; --i) {
        ++sum3;
        if (lines[x][i] >= lines[x][y]) {
            break;
        }
    }
    int sum4 = 0;
    for (int i = y + 1; i < n; ++i) {
        ++sum4;
        if (lines[x][i] >= lines[x][y]) {
            break;
        }
    }
    return sum1 * sum2 * sum3 * sum4;
}

void two(char lines[][BUFFER_SIZE], const int n_lines) {
    int max = 0;
    for (int i = 0; i < n_lines; ++i) {
        for (int j = 0; j < n_lines; ++j) {
            int sum = get_num_trees(lines, n_lines, i, j);
            if (sum > max) {
                max = sum;
            }
        }
    }
    printf("%d\n", max);
}

int main() {
    char fname[] = "08.txt";
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
    one(lines, n_lines);
    two(lines, n_lines);
}
