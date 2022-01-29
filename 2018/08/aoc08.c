#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 32768

long sum = 0;
int idx = 0;

void one(const int *arr, const int n_elements) {
    const int n_nodes = arr[idx++];
    const int n_meta = arr[idx++];
    for (int i = 0; i < n_nodes; ++i) {
        one(arr, n_elements);
    }
    for (int i = 0; i < n_meta; ++i) {
        sum += arr[idx];
        ++idx;
    }
}

int two(const int *arr, const int n_elements) {
    const int n_nodes = arr[idx++];
    const int n_meta = arr[idx++];
    int a[n_nodes];
    for (int i = 0; i < n_nodes; ++i) {
        a[i] = two(arr, n_elements);
    }
    int s = 0;
    for (int i = 0; i < n_meta; ++i) {
        if (n_nodes == 0) {
            s += arr[idx];
        } else {
            if (arr[idx] - 1 < n_nodes) {
                s += a[arr[idx] - 1];
            }
        }
        ++idx;
    }
    return s;
}

void read_input(const char *input) {
    int n_elements = 1;
    for (int i = 0; i < strlen(input); ++i) {
        if (input[i] == ' ') {
            ++n_elements;
        }
    }
    int arr[n_elements];
    char *line = strdup(input);
    char *orig = line;
    char *token;
    int i = 0;
    while ((token = strsep(&line, " "))) {
        arr[i++] = atoi(token);
    }
    free(orig);
    one(arr, n_elements);
    printf("%lu\n", sum);
    idx = 0;
    int res2 = two(arr, n_elements);
    printf("%d\n", res2);
}

int main() {
    char fname[] = "08.txt";
    FILE *f = fopen(fname, "r");
    if (f == NULL) {
        exit(1);
    }
    char buffer[BUFFER_SIZE];
    char *tmp = fgets(buffer, BUFFER_SIZE, f);
    fclose(f);
    if (tmp == NULL) {
        exit(2);
    }
    read_input(buffer);
}
