#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char input[] = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,2,19,6,23,1,23,5,27,1,9,27,31,1,31,10,35,2,35,9,39,1,5,39,43,2,43,9,47,1,5,47,51,2,51,13,55,1,55,10,59,1,59,10,63,2,9,63,67,1,67,5,71,2,13,71,75,1,75,10,79,1,79,6,83,2,13,83,87,1,87,6,91,1,6,91,95,1,10,95,99,2,99,6,103,1,103,5,107,2,6,107,111,1,10,111,115,1,115,5,119,2,6,119,123,1,123,5,127,2,127,6,131,1,131,5,135,1,2,135,139,1,139,13,0,99,2,0,14,0";

void print(int *arr, int count) {
    for (int i = 0; i < count; ++i) {
        printf("%d", arr[i]);
        if (i != count - 1) {
            printf(",");
        }
    }
    puts("");
}

void run(int *arr, int count) {
    for (int i = 0; i < count; ++i) {
        int n = arr[i];
        if (n == 1) {
            arr[arr[i + 3]] = arr[arr[i + 1]] + arr[arr[i + 2]];
            i += 3;
        } else if (n == 2) {
            arr[arr[i + 3]] = arr[arr[i + 1]] * arr[arr[i + 2]];
            i += 3;
        } else if (n == 99) {
            break;
        }
    }
}

void two(int *arr, int count, int *initial_arr) {
    for (int i = 0; i < 100; ++i) {
        for (int j = 0; j < 100; ++j) {
            memcpy(arr, initial_arr, sizeof(int) * count);
            arr[1] = i;
            arr[2] = j;
            run(arr, count);
            if (arr[0] == 19690720) {
                printf("%d\n", i * 100 + j);
            }
        }
    }
}

void one(int *arr, int count) {
    arr[1] = 12;
    arr[2] = 2;
    run(arr, count);
    printf("%d\n", arr[0]);
}

int main() {
    char *curr_line = strdup(input);
    char *line = curr_line;
    int count = 1;
    for (unsigned i = 0; i < strlen(input); ++i) {
        if (input[i] == ',') {
            ++count;
        }
    }
    int arr[count];
    int initial_arr[count];
    int i = 0;
    while (line) {
        char *token = strsep(&line, ",");
        arr[i++] = atoi(token);
    }
    free(curr_line);
    memcpy(initial_arr, arr, sizeof(int) * count);
    one(arr, count);
    two(arr, count, initial_arr);
}
