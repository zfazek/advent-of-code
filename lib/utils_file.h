#pragma once

#include <stdio.h>

int get_number_of_lines(FILE *f) {
    int ch;
    int n_lines = 0;
    while ((ch = fgetc(f)) != EOF) {
        if (ch == '\n') {
            ++n_lines;
        }
    }
    return n_lines;
}
