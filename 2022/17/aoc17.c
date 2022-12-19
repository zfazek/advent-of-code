#include "../../lib/utils_file.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int s1[] = {
    '@', '@', '@', '@',
    '.', '.', '.', '.',
    '.', '.', '.', '.',
    '.', '.', '.', '.',
};

int s2[] = {
    '.', '@', '.', '.',
    '@', '@', '@', '.',
    '.', '@', '.', '.',
    '.', '.', '.', '.',
};

int s3[] = {
    '@', '@', '@', '.',
    '.', '.', '@', '.',
    '.', '.', '@', '.',
    '.', '.', '.', '.',
};

int s4[] = {
    '@', '.', '.', '.',
    '@', '.', '.', '.',
    '@', '.', '.', '.',
    '@', '.', '.', '.',
};

int s5[] = {
    '@', '@', '.', '.',
    '@', '@', '.', '.',
    '.', '.', '.', '.',
    '.', '.', '.', '.',
};

#define BUFFER_SIZE 20000

#define N 1000000

char arr[N + 100][9];

typedef int Shape[16];

Shape shapes[5];

long sum = 0;

long aa = 1514285714288;

void print() {
    for (int i = sum + 7; i >= sum + 7 - 100; --i) {
        for (int j = 0; j < 9; ++j) {
            char c = arr[i][j];  
            printf("%c", c);
        }
        puts("");
    }
    puts("");
}

int can_move(int y, int x, int dx) {
    for (int i = 0; i < 4; ++i) {
        if (dx == -1) {
            for (int j = 0; j < 4; ++j) {
                if (arr[y + i][x + j] == '@' && arr[y + i][x + j + dx] == '#') {
                    return 0;
                }
            }
        } else {
            for (int j = 3; j >= 0; --j) {
                if (arr[y + i][x + j] == '@' && arr[y + i][x + j + dx] == '#') {
                    return 0;
                }
            }
        }
    }
    return 1;
}

int can_fall(int y, int x) {
    for (int i = 0; i < 4; ++i) {
        for (int j = 0; j < 4; ++j) {
            if (arr[y + i][x + j] == '@' && arr[y + i - 1][x + j] == '#') {
                return 0;
            }
        }
    }
    return 1;
}

void move(int y, int x, int dx) {
    //puts("MMM");
    for (int i = 0; i < 4; ++i) {
        if (dx == -1) {
            for (int j = 0; j < 4; ++j) {
                if (arr[y + i][x + j] == '@') {
                    arr[y + i][x + j + dx] = '@';
                    arr[y + i][x + j] = '.';
                }
            }
        } else {
            for (int j = 3; j >= 0; --j) {
                if (arr[y + i][x + j] == '@') {
                    arr[y + i][x + j + dx] = '@';
                    arr[y + i][x + j] = '.';
                }
            }
        }
    }
}

void fall(int y, int x) {
    //puts("FFF");
    for (int i = 0; i < 4; ++i) {
        for (int j = 0; j < 4; ++j) {
            if (arr[y + i][x + j] == '@') {
                arr[y + i - 1][x + j] = '@';
                arr[y + i][x + j] = '.';
            }
        }
    }
}

void store() {
    for (int i = 0; i < sum + 7; ++i) {
        for (int j = 1; j < 8; ++j) {
            if (arr[i][j] == '@') {
                arr[i][j] = '#';
            }
        }
    }
}

long get_max() {
    long max = 1;
    for (int i = 1; i < sum + 7; ++i) {
        int none = 0;
        for (int j = 1; j < 8; ++j) {
            if (arr[i][j] == '#') {
                max = i;
                none = 1;
            }
        }
        if (!none) {
            break;
        }
    }
    //printf("max: %d\n", max);
    return max;
}

void one(char lines[][BUFFER_SIZE], const int n_lines) {
    char c;
    int dx;
    int x;
    int y;
    for (int i = 0; i < 16; ++i) {
        shapes[0][i] = s1[i];
        shapes[1][i] = s2[i];
        shapes[2][i] = s3[i];
        shapes[3][i] = s4[i];
        shapes[4][i] = s5[i];
    }
    for (int i = 0; i < N; ++i) {
        if (i == 0) {
            c = '#';
        } else {
            c = '.';
        }
        for (int j = 1; j < 8; ++j) {
            arr[i][j] = c;
        }
        arr[i][0] = '#';
        arr[i][8] = '#';
    }
    char *l = lines[0];
    int length = strlen(l) - 1;
    //printf("%d\n", length);
    int fallen = 0;
    int l_idx = 0;
    long n = 0;
    while (n < 2022) {
        if (!fallen) {
            x = 3;
            y = sum + 4;
            int idx = n % 5;
            for (int i = 0; i < 4; ++i) {
                for (int j = 0; j < 4; ++j) {
                    arr[sum + 4 + i][j + 3] = shapes[idx][i * 4 + j];
                }
            }
            //print();
        }
        if (l[l_idx++ % length] == '<') {
            dx = -1;
        } else {
            dx = 1;
        }
        //printf("dx: %d\n", dx);
        if (can_move(y, x, dx)) {
            move(y, x, dx);
            //print();
            x += dx;
        } 
        if (can_fall(y, x)) {
            fall(y, x);
            --y;
            fallen = 1;
            //print();
        } else {
            store();
            ++n;
            fallen = 0;
            sum = get_max();
        }
    }
    //print();
    printf("%ld\n", sum);
}

int main() {
    char fname[] = "17.txt";
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
