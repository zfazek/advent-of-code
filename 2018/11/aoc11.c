#include <stdio.h>

#define SIZE 300

// Find the fuel cell's rack ID, which is its X coordinate plus 10.
// Begin with a power level of the rack ID times the Y coordinate.
// Increase the power level by the value of the grid serial number (your puzzle
// input). Set the power level to itself multiplied by the rack ID. Keep only
// the hundreds digit of the power level (so 12345 becomes 3; numbers with no
// hundreds digit become 0). Subtract 5 from the power level.

int dp[SIZE * SIZE * SIZE];

int get_power(int x, int y, int n) {
    int v = 0;
    int rack_id = x + 10;
    v = rack_id * y;
    v += n;
    v *= rack_id;
    v = v / 100;
    v %= 10;
    v -= 5;
    return v;
}

void foo(int n, int s) {
    int arr[SIZE][SIZE];
    for (int y = 0; y < SIZE; ++y) {
        for (int x = 0; x < SIZE; ++x) {
            arr[y][x] = get_power(x + 1, y + 1, n);
        }
    }
    int max = 0;
    int max_x = 0;
    int max_y = 0;
    int best_size = 0;
    int min_size = 1;
    int max_size = s;
    if (s == 0) {
        max_size = SIZE;
    }
    for (int size = min_size; size <= max_size; ++size) {
        for (int y = 0; y < SIZE - size; ++y) {
            for (int x = 0; x < SIZE - size; ++x) {
                int power = dp[y * SIZE * SIZE + x * SIZE + size - 1];
                for (int dy = 0; dy < size; ++dy) {
                    power += arr[y + dy][x + size - 1];
                }
                for (int dx = 0; dx < size - 1; ++dx) {
                    power += arr[y + size - 1][x + dx];
                }
                dp[y * SIZE * SIZE + x * SIZE + size] = power;
                if (size == min_size && x == 0 && y == 0) {
                    max = power;
                    max_x = x;
                    max_y = y;
                    best_size = size;
                } else {
                    if (power > max) {
                        max = power;
                        max_x = x;
                        max_y = y;
                        best_size = size;
                    }
                }
            }
        }
    }
    printf("serial number: %d, (%d,%d,%d): %d\n", n, max_x + 1, max_y + 1,
           best_size, max);
}

int main() {
    foo(9221, 3);
    foo(9221, 0);
}
