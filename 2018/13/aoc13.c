#include "../../lib/utils_file.h"
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 200

#define LEFT 0
#define STRAIGHT 1
#define RIGHT 2

int first = 1;

typedef struct Cart {
    int x;
    int y;
    char dir;
    int turn;
    int live;
} Cart;

int compare_cart(const void *a, const void *b) {
    Cart *c1 = (Cart *)a;
    Cart *c2 = (Cart *)b;
    if (c1->live != c2->live)
        return c2->live - c1->live;
    if (c1->y != c2->y)
        return c1->y - c2->y;
    return c1->x - c2->x;
}

int get_number_of_carts(Cart *carts, int n_cars) {
    int count = 0;
    for (int i = 0; i < n_cars; ++i) {
        if (carts[i].live) {
            ++count;
        }
    }
    return count;
}
char get_cart(int x, int y, Cart *carts, int n_carts) {
    for (int i = 0; i < n_carts; ++i) {
        Cart *cart = &carts[i];
        if (cart->live && cart->x == x && cart->y == y) {
            return cart->dir;
        }
    }
    return ' ';
}

void print_map(char lines[][BUFFER_SIZE], int n_lines, Cart *carts,
               int n_carts) {
    return;
    printf("number of carts: %d\n", get_number_of_carts(carts, n_carts));
    for (int i = 0; i < n_carts; ++i) {
        Cart *cart = &carts[i];
        printf("cart[%d].live: %d: %d,%d\n", i, cart->live, cart->y, cart->x);
    }
    for (int i = 0; i < n_lines; ++i) {
        for (unsigned j = 0; j < strlen(lines[i]); ++j) {
            char c = get_cart(j, i, carts, n_carts);
            if (c != ' ') {
                printf("%c", c);
            } else {
                printf("%c", lines[i][j]);
            }
        }
    }
    puts("");
    fflush(stdout);
}

int is_crashed(Cart *carts, int n_carts, int *x, int *y) {
    for (int i = 0; i < n_carts; ++i) {
        for (int j = i + 1; j < n_carts; ++j) {
            Cart *c1 = &carts[i];
            Cart *c2 = &carts[j];
            if (!c1->live || !c2->live) {
                continue;
            }
            if (c1->x == c2->x && c1->y == c2->y) {
                *x = c1->x;
                *y = c1->y;
                c1->live = 0;
                c2->live = 0;
                return 1;
            }
        }
    }
    return 0;
}

void tick(char lines[][BUFFER_SIZE], int n_lines, Cart *carts, int n_carts) {
    qsort(carts, n_carts, sizeof(Cart), compare_cart);
    print_map(lines, n_lines, carts, n_carts);
    for (int i = 0; i < n_carts; ++i) {
        Cart *cart = &carts[i];
        if (!cart->live) {
            continue;
        }
        char c = cart->dir;
        if (c == 'v') {
            ++cart->y;
            char m = lines[cart->y][cart->x];
            if (m == '\\') {
                cart->dir = '>';
            } else if (m == '/') {
                cart->dir = '<';
            } else if (m == '+') {
                if (cart->turn == LEFT) {
                    cart->dir = '>';
                    cart->turn = STRAIGHT;
                } else if (cart->turn == STRAIGHT) {
                    cart->turn = RIGHT;
                } else if (cart->turn == RIGHT) {
                    cart->dir = '<';
                    cart->turn = LEFT;
                }
            }
        } else if (c == '^') {
            --cart->y;
            char m = lines[cart->y][cart->x];
            if (m == '\\') {
                cart->dir = '<';
            } else if (m == '/') {
                cart->dir = '>';
            } else if (m == '+') {
                if (cart->turn == LEFT) {
                    cart->dir = '<';
                    cart->turn = STRAIGHT;
                } else if (cart->turn == STRAIGHT) {
                    cart->turn = RIGHT;
                } else if (cart->turn == RIGHT) {
                    cart->dir = '>';
                    cart->turn = LEFT;
                }
            }
        } else if (c == '<') {
            --cart->x;
            char m = lines[cart->y][cart->x];
            if (m == '\\') {
                cart->dir = '^';
            } else if (m == '/') {
                cart->dir = 'v';
            } else if (m == '+') {
                if (cart->turn == LEFT) {
                    cart->dir = 'v';
                    cart->turn = STRAIGHT;
                } else if (cart->turn == STRAIGHT) {
                    cart->turn = RIGHT;
                } else if (cart->turn == RIGHT) {
                    cart->dir = '^';
                    cart->turn = LEFT;
                }
            }
        } else if (c == '>') {
            ++cart->x;
            char m = lines[cart->y][cart->x];
            if (m == '\\') {
                cart->dir = 'v';
            } else if (m == '/') {
                cart->dir = '^';
            } else if (m == '+') {
                if (cart->turn == LEFT) {
                    cart->dir = '^';
                    cart->turn = STRAIGHT;
                } else if (cart->turn == STRAIGHT) {
                    cart->turn = RIGHT;
                } else if (cart->turn == RIGHT) {
                    cart->dir = 'v';
                    cart->turn = LEFT;
                }
            }
        }
        int x;
        int y;
        if (is_crashed(carts, n_carts, &x, &y) && first) {
            first = 0;
            printf("%d,%d\n", x, y);
        }
    }
}

void one(char lines[][BUFFER_SIZE], int n_lines, Cart *carts, int n_carts) {
    while (get_number_of_carts(carts, n_carts) > 1) {
        tick(lines, n_lines, carts, n_carts);
    }
    for (int i = 0; i < n_carts; ++i) {
        Cart *cart = &carts[i];
        if (cart->live) {
            printf("%d,%d\n", cart->x, cart->y);
        }
    }
}

int main() {
    char fname[] = "13.txt";
    FILE *f = fopen(fname, "r");
    if (f == NULL) {
        exit(1);
    }
    int n_lines = get_number_of_lines(f);
    fseek(f, 0, SEEK_SET);
    char lines[n_lines][BUFFER_SIZE];
    char buffer[BUFFER_SIZE];
    int idx = 0;
    int n_carts = 0;
    while (fgets(buffer, BUFFER_SIZE, f)) {
        strcpy(lines[idx++], buffer);
        for (unsigned x = 0; x < strlen(buffer); ++x) {
            char c = buffer[x];
            if (c == 'v' || c == '^' || c == '<' || c == '>') {
                ++n_carts;
            }
        }
    }
    Cart carts[n_carts];
    idx = 0;
    for (int i = 0; i < n_lines; ++i) {
        for (unsigned x = 0; x < strlen(buffer); ++x) {
            char c = lines[i][x];
            if (c == 'v' || c == '^' || c == '<' || c == '>') {
                Cart cart;
                cart.x = x;
                cart.y = i;
                cart.dir = c;
                cart.turn = LEFT;
                cart.live = 1;
                if (c == 'v') {
                    lines[i][x] = '|';
                } else if (c == '^') {
                    lines[i][x] = '|';
                } else if (c == '<') {
                    lines[i][x] = '-';
                } else if (c == '>') {
                    lines[i][x] = '-';
                }
                carts[idx++] = cart;
            }
        }
    }
    fclose(f);
    one(lines, n_lines, carts, n_carts);
}
