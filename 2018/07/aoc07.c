#include "../../lib/utils_file.h"
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 50

// Step C must be finished before step A can begin.
// Step C must be finished before step F can begin.
// Step A must be finished before step B can begin.
// Step A must be finished before step D can begin.
// Step B must be finished before step E can begin.
// Step D must be finished before step E can begin.
// Step F must be finished before step E can begin.

typedef struct Step {
    int before;
    int after;
} Step;

char arr[26];

void read_input(Step *steps, const char lines[][BUFFER_SIZE],
                const int n_lines) {
    for (int i = 0; i < n_lines; ++i) {
        const char *line = lines[i];
        char before = line[5];
        char after = line[36];
        Step step = {before - 'A', after - 'A'};
        steps[i] = step;
        arr[before - 'A'] = 1;
        arr[after - 'A'] = 1;
    }
}

int get_next(const Step *steps, const int n_lines) {
    for (int i = 0; i < 26; ++i) {
        if (arr[i] == 0) {
            continue;
        }
        int good = 1;
        for (int j = 0; j < n_lines; ++j) {
            if (arr[steps[j].before] == 0) {
                continue;
            }
            if (steps[j].after == i) {
                good = 0;
                break;
            }
        }
        if (good) {
            return i;
        }
    }
    return -1;
}

void one(const char lines[][BUFFER_SIZE], const int n_lines) {
    int next;
    Step steps[n_lines];
    read_input(steps, lines, n_lines);
    while ((next = get_next(steps, n_lines)) != -1) {
        printf("%c", next + 'A');
        arr[next] = 0;
    }
}

int main() {
    char fname[] = "07.txt";
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
