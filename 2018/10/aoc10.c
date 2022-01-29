#include "../../lib/utils_file.h"
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 50

// position=< 9,  1> velocity=< 0,  2>
// position=< 7,  0> velocity=<-1,  0>
// position=< 3, -2> velocity=<-1,  1>

int min_x;
int max_x;
int min_y;
int max_y;

typedef struct Point {
    int x;
    int y;
    int dx;
    int dy;
} Point;

void set_min_max(Point *points, int n_lines, int i) {
    min_x = points[0].x;
    max_x = points[0].x;
    min_y = points[0].y;
    max_y = points[0].y;
    for (int i = 1; i < n_lines; ++i) {
        Point *point = &points[i];
        if (point->x < min_x)
            min_x = point->x;
        if (point->x > max_x)
            max_x = point->x;
        if (point->y < min_y)
            min_y = point->y;
        if (point->y > max_y)
            max_y = point->y;
    }
    // printf("i: %d, x: (%d, %d), y: (%d, %d), dy: %d\n", i, min_x, max_x,
    // min_y, max_y, max_y - min_y);
}

void read_input(Point *points, char lines[][BUFFER_SIZE], int n_lines) {
    for (int i = 0; i < n_lines; ++i) {
        char *line = lines[i];
        Point point;
        point.x = atoi(strchr(line, '<') + 1);
        point.y = atoi(strchr(line, ',') + 1);
        point.dx = atoi(strchr(line, 'y') + 3);
        point.dy = atoi(strchr(line, 'y') + 6);
        points[i] = point;
    }
}

int found(Point *points, int n_lines, int x, int y) {
    for (int i = 0; i < n_lines; ++i) {
        Point *point = &points[i];
        if (point->x == x && point->y == y) {
            return 1;
        }
    }
    return 0;
}

void print_points(Point *points, int n_lines, int n) {
    printf("%d\n", n);
    for (int i = min_y; i <= max_y; ++i) {
        for (int j = min_x; j <= max_x; ++j) {
            if (found(points, n_lines, j, i)) {
                printf("%c", '#');
            } else {
                printf("%c", '.');
            }
        }
        puts("");
    }
}

void one(char lines[][BUFFER_SIZE], int n_lines) {
    Point points[n_lines];
    read_input(points, lines, n_lines);
    int i = 0;
    set_min_max(points, n_lines, i);
    int prev_max_dy = max_y - min_y;
    while (1) {
        for (int j = 0; j < n_lines; ++j) {
            Point *point = &points[j];
            point->x += point->dx;
            point->y += point->dy;
        }
        set_min_max(points, n_lines, i);
        int max_dy = max_y - min_y;
        if (max_dy > prev_max_dy) {
            for (int j = 0; j < n_lines; ++j) {
                Point *point = &points[j];
                point->x -= point->dx;
                point->y -= point->dy;
            }
            set_min_max(points, n_lines, i);
            print_points(points, n_lines, i);
            break;
        }
        prev_max_dy = max_dy;
        ++i;
    }
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
    one(lines, n_lines);
}
