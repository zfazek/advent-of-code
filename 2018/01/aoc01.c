#include "../../lib/klib/kbtree.h"
#include "../../lib/utils_file.h"

typedef int elem_t;

#define elem_cmp(a, b) (a - b)
KBTREE_INIT(i32, elem_t, elem_cmp)

void one(const int *numbers, const int n_lines) {
    int n = 0;
    for (int i = 0; i < n_lines; ++i) {
        n += numbers[i];
    }
    printf("%d\n", n);
}

void two(const int *numbers, const int n_lines) {
    kbtree_t(i32) * seen;
    elem_t *p, t;
    kbitr_t itr;
    seen = kb_init(i32, KB_DEFAULT_SIZE);

    int n = 0;
    int iter = 0;
    while (1) {
        for (int i = 0; i < n_lines; ++i) {
            const int num = numbers[i];
            n += num;
            t = n;
            p = kb_getp(i32, seen, &t);
            if (p) {
                printf("%d\n", *p);
                exit(0);
            } else {
                kb_putp(i32, seen, &t);
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
    char buffer[10];
    while (fgets(buffer, 10, f)) {
        const int n = atoi(buffer);
        numbers[idx++] = n;
    }
    fclose(f);
    one(numbers, n_lines);
    two(numbers, n_lines);
}
