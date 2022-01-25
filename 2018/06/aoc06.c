#include "../../lib/utils_ds.h"
#include "../../lib/utils_file.h"
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 10

// 1, 1
// 1, 6
// 8, 3
// 3, 4
// 5, 5
// 8, 9

typedef struct Point {
    int x;
    int y;
} Point;

int dp[500];

void read_input(Point *points, int *max_x, int *max_y,
                const char lines[][BUFFER_SIZE], const int n_lines) {
    for (int i = 0; i < n_lines; ++i) {
        char *tmp = strdup(lines[i]);
        char *line = tmp;
        const int x = atoi(strsep(&line, " "));
        const int y = atoi(line);
        Point point = {x, y};
        points[i] = point;
        free(tmp);
        if (x > *max_x) {
            *max_x = x;
        }
        if (y > *max_y) {
            *max_y = y;
        }
    }
    ++*max_x;
    ++*max_y;
}

void set_arr(Point *points, int *arr, const int max_x, const int max_y,
             const int n_lines) {
    for (int y = 0; y < max_y; ++y) {
        for (int x = 0; x < max_x; ++x) {
            int min_manhattan = 2 * max_x * max_y;
            int min_i = 0;
            for (int i = 0; i < n_lines; ++i) {
                const Point point = points[i];
                const int manhattan = abs(x - point.x) + abs(y - point.y);
                if (manhattan < min_manhattan) {
                    min_manhattan = manhattan;
                    min_i = i + 1;
                } else if (manhattan == min_manhattan) {
                    min_i = 0;
                }
            }
            arr[y * max_x + x] = min_i;
        }
    }
}

void print_arr(const int *arr, const int max_x, const int max_y) {
    for (int y = 0; y < max_y; ++y) {
        for (int x = 0; x < max_x; ++x) {
            printf("%d", arr[y * max_x + x]);
        }
        puts("");
    }
    puts("");
}

int is_infinite(const int *arr, const int max_x, const int max_y, const int v) {
    for (int i = 0; i < max_y; ++i) {
        if (arr[i * max_x] == v || arr[i * max_x + max_x - 1] == v) {
            return 1;
        }
    }
    for (int j = 0; j < max_x; ++j) {
        if (arr[j] == v || arr[max_y * max_x - max_x + j] == v) {
            return 1;
        }
    }
    return 0;
}

int get_area(const int *arr, const int max_x, const int max_y, const int v) {
    if (dp[v] > 0) {
        return dp[v];
    }
    int sum = 0;
    for (int y = 1; y < max_y - 1; ++y) {
        for (int x = 1; x < max_x - 1; ++x) {
            if (arr[y * max_x + x] == v) {
                ++sum;
            }
        }
    }
    return sum;
}

int get_largest(const int *arr, const int max_x, const int max_y) {
    int largest = 0;
    for (int y = 1; y < max_y - 1; ++y) {
        for (int x = 1; x < max_x - 1; ++x) {
            const int v = arr[y * max_x + x];
            if (is_infinite(arr, max_x, max_y, v)) {
                continue;
            }
            const int area = get_area(arr, max_x, max_y, v);
            dp[v] = area;
            if (area > largest) {
                largest = area;
            }
        }
    }
    return largest;
}

int get_size(const int max_x, const int max_y, const Point *points,
             const int n_lines, const int max_manhattan) {
    int count = 0;
    for (int y = 0; y < max_y; ++y) {
        for (int x = 0; x < max_x; ++x) {
            int sum = 0;
            for (int i = 0; i < n_lines; ++i) {
                sum += abs(x - points[i].x) + abs(y - points[i].y);
            }
            if (sum < max_manhattan) {
                ++count;
            }
        }
    }
    return count;
}

void one(const char lines[][BUFFER_SIZE], const int n_lines) {
    int max_x = 0;
    int max_y = 0;
    Point points[n_lines];
    read_input(points, &max_x, &max_y, lines, n_lines);
    int arr[max_x * max_y];
    memset(arr, 0, sizeof(int) * max_x * max_y);
    set_arr(points, arr, max_x, max_y, n_lines);
    // print_arr(arr, max_x, max_y);
    const int largest = get_largest(arr, max_x, max_y);
    printf("%d\n", largest);
    const int max_manhattan = 10000;
    const int size = get_size(max_x, max_y, points, n_lines, max_manhattan);
    printf("%d\n", size);
}

int main() {
    char fname[] = "06.txt";
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
