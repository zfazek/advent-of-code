#include "../../lib/klib/kbtree.h"
#include "../../lib/utils_file.h"

#define BUFFER_SIZE 10

#define elem_cmp(a, b) (a - b)
KBTREE_INIT(i32, int, elem_cmp)

void one(int *numbers, int n_lines) {
    int n = 0;
    for (int i = 0; i < n_lines; ++i) {
        n += numbers[i];
    }
    printf("%d\n", n);
}

void two(int *numbers, int n_lines) {
    kbtree_t(i32) *seen = kb_init(i32, KB_DEFAULT_SIZE);
    int n = 0;
    while (1) {
        for (int i = 0; i < n_lines; ++i) {
            int num = numbers[i];
            n += num;
            if (kb_getp(i32, seen, &n)) {
                printf("%d\n", n);
                exit(0);
            } else {
                kb_putp(i32, seen, &n);
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
