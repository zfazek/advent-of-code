#include "../../lib/utils_file.h"
#include "../../lib/utils_ds.h"
#include <stdio.h>

#define BUFFER_SIZE 100

typedef struct Dir {
    char name[100];
    struct Dir *parent;
    struct Dir *dirs[10];
    int idx_dirs;
    int sum;
} Dir;

Dir *root = NULL;
int sum = 0;
int result = 70000000;

void print(const Dir *d) {
    printf("### Dir name: \"%s\", parent: \"%s\", sum: %d, n: %d, ",
            d->name, d->parent->name, d->sum, d->idx_dirs);
    for (int i = 0; i < d->idx_dirs; ++i) {
        printf("%s ", d->dirs[i]->name);
    }
    puts("");
}

void foo(Dir *d) {
    if (d == NULL) {
        return;
    }
    if (d->sum <= 100000) {
        sum += d->sum;
    }
    for (int i = 0; i < d->idx_dirs; ++i) {
        foo(d->dirs[i]);
    }
}

void foo1(Dir *d, const int min) {
    if (d == NULL) {
        return;
    }
    if (d->sum > min && d->sum < result) {
        result = d->sum;
    }
    for (int i = 0; i < d->idx_dirs; ++i) {
        foo1(d->dirs[i], min);
    }
}

void one(char lines[][BUFFER_SIZE], const int n_lines) {
    Dir *d = root;
    for (int i = 0; i < n_lines; ++i) {
        char *line = lines[i];
        line[strlen(line) - 1] = 0;
        const char first = line[0];
        const char third = line[2];
        const char sixth = line[5];
        if (first == '$' && third == 'c') {
            Dir *dir = (Dir*)malloc(sizeof(Dir));
            if (sixth != '.') {
                dir->idx_dirs = 0;
                dir->sum = 0;
                if (root == NULL) {
                    root = dir;
                    dir->parent = NULL;
                } else {
                    d->dirs[d->idx_dirs++] = dir;
                    dir->parent = d;
                }
                strcpy(dir->name, line + 5);
                d = dir;
                continue;
            } else {
                d->parent->sum += d->sum;
                d = d ->parent;
                continue;
            }
        }
        if (first >= '0' && first <= '9') {
            d->sum += atoi(line);
            continue;
        }
    }
    while (d->parent != NULL) {
        d->parent->sum += d->sum;
        d = d->parent;
    }
    foo(root);
    printf("%d\n", sum);
}

void two() {
    int unused = 70000000 - root->sum;
    int min = 30000000 - unused;
    foo1(root, min);
    printf("%d\n", result);
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
    two();
}
