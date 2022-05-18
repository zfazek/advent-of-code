#include <stdio.h>

void one() {
    char buffer[7];
    int n = 0;
    for (int i = 138307; i <= 654504; ++i) {
        sprintf(buffer, "%d", i);
        int decrease = 0;
        for (int j = 0; j < 5; ++j) {
            if (buffer[j + 1] < buffer[j]) {
                decrease = 1;
                break;
            }
        }
        if (decrease) {
            continue;
        }
        int same_digits = 0;
        for (int j = 0; j < 5; ++j) {
            if (buffer[j + 1] == buffer[j]) {
                same_digits = 1;
                break;
            }
        }
        if (!same_digits) {
            continue;
        }
        ++n;

    }
    printf("%d\n", n);
}

int is_ok(int n) {
    static char buffer[7];
    sprintf(buffer, "%d", n);
    for (int i = 0; i < 5; ++i) {
        if (buffer[i + 1] < buffer[i]) {
            return 0;
        }
    }
    int digits = 1;
    for (int i = 0; i < 5; ++i) {
        if (buffer[i + 1] == buffer[i]) {
            ++digits;
        } else {
            if (digits == 2) {
                return 1;
            } else {
                digits = 1;
            }
        }
    }
    if (digits == 2) {
        return 1;
    }
    return 0;
}

void two() {
    int n = 0;
    for (int i = 138307; i <= 654504; ++i) {
        if (is_ok(i)) {
            ++n;
        }

    }
    printf("%d\n", n);
}

int main() {
//    int n;
//    n = 112233; printf("%d %d\n", n, is_ok(n));
//    n = 112222; printf("%d %d\n", n, is_ok(n));
//    n = 111222; printf("%d %d\n", n, is_ok(n));
//    n = 123444; printf("%d %d\n", n, is_ok(n));
//    n = 111122; printf("%d %d\n", n, is_ok(n));
//    n = 122233; printf("%d %d\n", n, is_ok(n));
//    n = 122344; printf("%d %d\n", n, is_ok(n));
    one();
    two();
}
