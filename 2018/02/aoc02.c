#include "../../lib/utils_file.h"
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 28

void one(const char words[][BUFFER_SIZE], const int n_lines) {
    int n2 = 0;
    int n3 = 0;
    for (int l = 0; l < n_lines; ++l) {
        int arr[26] = {0};
        for (int i = 0; i < 26; ++i) {
            const int idx = words[l][i] - 'a';
            ++arr[idx];
        }
        int found_2 = 0;
        int found_3 = 0;
        for (int i = 0; i < 26; ++i) {
            if (arr[i] == 2) {
                found_2 = 1;
            } else if (arr[i] == 3) {
                found_3 = 1;
            }
        }
        if (found_2) {
            ++n2;
        }
        if (found_3) {
            ++n3;
        }
    }
    printf("%d\n", n2 * n3);
}

void two(const char words[][BUFFER_SIZE], const int n_lines) {
    for (int i = 0; i < n_lines; ++i) {
        for (int j = i + 1; j < n_lines; ++j) {
            int count = 0;
            for (int k = 0; k < 26 && count < 2; ++k) {
               const char c1 = words[i][k];
               const char c2 = words[j][k];
               if (c1 != c2) {
                   ++count;
               }
            }
            if (count == 1) {
                for (int k = 0; k < 26; ++k) {
                    const char c1 = words[i][k];
                    const char c2 = words[j][k];
                    if (c1 == c2) {
                        printf("%c", c1);
                    }
                }
                puts("");
                exit(0);
            }
        }
    }
}

int main() {
    char fname[] = "02.txt";
    FILE *f = fopen(fname, "r");
    if (f == NULL) {
        exit(1);
    }
    int n_lines = get_number_of_lines(f);
    fseek(f, 0, SEEK_SET);
    char words[n_lines][BUFFER_SIZE];
    char buffer[BUFFER_SIZE];
    int idx = 0;
    while (fgets(buffer, BUFFER_SIZE, f)) {
        strcpy(words[idx++], buffer);
    }
    fclose(f);
    one(words, n_lines);
    two(words, n_lines);
}
