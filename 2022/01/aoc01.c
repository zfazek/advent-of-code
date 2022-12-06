#include "../../lib/utils_file.h"
#include "../../lib/utils_ds.h"
#include <stdio.h>

#define BUFFER_SIZE 10

void one(char lines[][BUFFER_SIZE], const int n_lines) {
    int sums[n_lines];
    int j = 0;
    int sum = 0;
    for (int i = 0; i < n_lines; ++i) {
        if (strlen(lines[i]) == 1) {
            sums[j++] = sum;
            sum = 0;
            continue;
        }
        sum += atoi(lines[i]);
    }
    qsort(sums, j, sizeof(int), compare_int_rev);
    printf("%d\n", sums[0]);
    printf("%d\n", sums[0] + sums[1] + sums[2]);
}

int main() {
    char fname[] = "01.txt";
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
