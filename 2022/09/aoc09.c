#include "../../lib/utils_file.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 6

#define N 500

typedef struct Coord {
    int x;
    int y;
} Coord;

int dx, dy;
int M;

char visited[N][N];
Coord r[10];

void print() {
    for (int i = 0; i < N; ++i) {
        for (int j = 0; j < N; ++j) {
            int found = 0;
            for (int m = 0; m < M; ++m) {
                if (found == 0 && r[m].x == j && r[m].y == i) {
                    printf("%c", m == 0 ? 'H' : '0' + m);
                    found = 1;
                }
            }
            if (found == 0) {
                printf("%c", '.');
            } else {
            
            }
        }
        puts("");
    }
    puts("");
}

void print_visited() {
    for (int i = 0; i < N; ++i) {
        for (int j = 0; j < N; ++j) {
            printf("%d", visited[i][j]);
        }
        puts("");
    }
    puts("");
}

void init() {
    for (int i = 0; i < M; ++i) {
        r[i].x = N / 2;
        r[i].y = N / 2;
    }
    for (int i = 0; i < N; ++i) {
        for (int j = 0; j < N; ++j) {
            visited[i][j] = 0;
        }
    }
}

int should_move(char lines[][BUFFER_SIZE], const int n_lines, const int m) {
    if (abs(r[m].x - r[m - 1].x) > 1) return 1;
    if (abs(r[m].y - r[m - 1].y) > 1) return 1;
    return 0;
}


void move(char lines[][BUFFER_SIZE], const int n_lines, const int m) {
    if (should_move(lines, n_lines, m)) {
        if (r[m - 1].x - r[m].x > 1) {
            ++r[m].x;
            if (r[m - 1].y > r[m].y) {
                ++r[m].y;
            }
            if (r[m - 1].y < r[m].y) {
                --r[m].y;
            }
        }
        if (r[m].x - r[m - 1].x > 1) {
            --r[m].x;
            if (r[m - 1].y > r[m].y) {
                ++r[m].y;
            }
            if (r[m - 1].y < r[m].y) {
                --r[m].y;
            }
        }
        if (r[m - 1].y - r[m].y > 1) {
            ++r[m].y;
            if (r[m - 1].x > r[m].x) {
                ++r[m].x;
            }
            if (r[m - 1].x < r[m].x) {
                --r[m].x;
            }
        }
        if (r[m].y - r[m - 1].y > 1) {
            --r[m].y;
            if (r[m - 1].x > r[m].x) {
                ++r[m].x;
            }
            if (r[m - 1].x < r[m].x) {
                --r[m].x;
            }
        }
    }
}

void one(char lines[][BUFFER_SIZE], const int n_lines) {
    init();
    visited[r[M - 1].y][r[M - 1].x] = 1;
    for (int i = 0; i < n_lines; ++i) {
        // printf("%s", lines[i]);
        const char c1 = lines[i][0];
        const int n = atoi(lines[i] + 2);
        if (c1 == 'R') {
            dx = 1;
            dy = 0;
        } else if (c1 == 'L') {
            dx = -1;
            dy = 0;
        } else if (c1 == 'U') {
            dx = 0;
            dy = -1;
        } else {
            dx = 0;
            dy = 1;
        }
        for (int j = 0; j < n; ++j) {
            r[0].x += dx;
            r[0].y += dy;
            for (int m = 1; m < M; ++m) {
                move(lines, n_lines, m);
            }
            // print();
            visited[r[M - 1].y][r[M - 1].x] = 1;
        }
        // print();
    }
    int sum = 0;
    for (int i = 0; i < N; ++i) {
        for (int j = 0; j < N; ++j) {
            sum += visited[i][j];
        }
    }
    // print_visited();
    printf("%d\n", sum);
}

int main() {
    char fname[] = "09.txt";
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
    M = 2;
    one(lines, n_lines);
    M = 10;
    one(lines, n_lines);
}
