#include "../../lib/utils_file.h"
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 28

// #1 @ 1,3: 4x4
// #2 @ 3,1: 4x4
// #3 @ 5,5: 2x2

int is_all_one(int *arr, int max_x, int x, int y, int dx, int dy) {
    for (int i = y; i < y + dy; ++i) {
        for (int j = x; j < x + dx; ++j) {
            int idx = i * max_x + j;
            if (arr[idx] != 1) {
                return 0;
            }
        }
    }
    return 1;
}

void set_maximums(char lines[][BUFFER_SIZE], const int n_lines, int *max_x, int *max_y) {
    for (int i = 0; i < n_lines; ++i) {
        char *curr_line = strdup(lines[i]);
        char *line = curr_line;
        char *token = strsep(&line, "@");
        char *coords = strsep(&line, ":");
        char *x_coord = strsep(&coords, ",");
        char *y_coord = coords;
        char *dx_coord = strsep(&line, "x");
        char *dy_coord = line;
        int x = atoi(x_coord);
        int y = atoi(y_coord);
        int dx = atoi(dx_coord);
        int dy = atoi(dy_coord);
        if (x + dx > *max_x) {
            *max_x = x + dx;
        }
        if (y + dy > *max_y) {
            *max_y = y + dy;
        }
        free(curr_line);
    }
    ++*max_x;
    ++*max_y;
}

void fill_arrays(char lines[][BUFFER_SIZE], const int n_lines, int *arr, int max_x) {
    for (int i = 0; i < n_lines; ++i) {
        char *curr_line = strdup(lines[i]);
        char *line = curr_line;
        char *token = strsep(&line, "@");
        int claim = atoi(token + 1);
        char *coords = strsep(&line, ":");
        char *x_coord = strsep(&coords, ",");
        char *y_coord = coords;
        char *dx_coord = strsep(&line, "x");
        char *dy_coord = line;
        int x = atoi(x_coord);
        int y = atoi(y_coord);
        int dx = atoi(dx_coord);
        int dy = atoi(dy_coord);
        free(curr_line);
        for (int i = y; i < y + dy; ++i) {
            for (int j = x; j < x + dx; ++j) {
                int idx = i * max_x + j;
                ++arr[idx];
            }
        }
    }
}

long get_result_one(int *arr, int max_x, int max_y) {
    long n = 0;
    for (int i = 0; i < max_y; ++i) {
        for (int j = 0; j < max_x; ++j) {
            int idx = i * max_x + j;
            if (arr[idx] > 1) {
                ++n;
            }
        }
    }
    return n;
}

int get_result_two(char lines[][BUFFER_SIZE], const int n_lines, int *arr, int max_x, int max_y) {
    for (int i = 0; i < n_lines; ++i) {
        char *curr_line = strdup(lines[i]);
        char *line = curr_line;
        char *token = strsep(&line, "@");
        int claim = atoi(token + 1);
        char *coords = strsep(&line, ":");
        char *x_coord = strsep(&coords, ",");
        char *y_coord = coords;
        char *dx_coord = strsep(&line, "x");
        char *dy_coord = line;
        int x = atoi(x_coord);
        int y = atoi(y_coord);
        int dx = atoi(dx_coord);
        int dy = atoi(dy_coord);
        free(curr_line);
        if (is_all_one(arr, max_x, x, y, dx, dy)) {
            return claim;
        }
    }
    return 0;
}

int main() {
    char fname[] = "03.txt";
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
    int max_x = 0;
    int max_y = 0;
    set_maximums(lines, n_lines, &max_x, &max_y);
    int *arr = (int*)calloc(max_x * max_y, sizeof(int));
    fill_arrays(lines, n_lines, arr, max_x);
    printf("%lu\n", get_result_one(arr, max_x, max_y));
    printf("%d\n", get_result_two(lines, n_lines, arr, max_x, max_y));
    free(arr);
}
