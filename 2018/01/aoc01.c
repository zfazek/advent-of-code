#include <string.h>

#include "../../lib/utils_file.h"
#include "../../lib/utils_hash.h"

#define BUFFER_SIZE 10

void one(int *numbers, int n_lines) {
    int n = 0;
    for (int i = 0; i < n_lines; ++i) {
        n += numbers[i];
    }
    printf("%d\n", n);
    fflush(stdout);
}

void two(int *numbers, int n_lines) {
    int n = 0;
    Hash hash[HASH_SIZE];
    memset(hash, 0, sizeof(Hash) * HASH_SIZE);
    while (1) {
        for (int i = 0; i < n_lines; ++i) {
            n += numbers[i];
            if (hash_contains(hash, n)) {
                printf("%d\n", n);
                exit(0);
            } else {
                hash_insert(hash, n);
            }
        }
    }
}

int main() {
    char fname[] = "01.txt";
    FILE *f = fopen(fname, "r");
    if (f == NULL) {
        exit(1);
    }
    int n_lines = get_number_of_lines(f);
    fseek(f, 0, SEEK_SET);
    int numbers[n_lines];
    int idx = 0;
    char buffer[BUFFER_SIZE];
    while (fgets(buffer, BUFFER_SIZE, f)) {
        numbers[idx++] = atoi(buffer);
    }
    fclose(f);
    one(numbers, n_lines);
    two(numbers, n_lines);
}
