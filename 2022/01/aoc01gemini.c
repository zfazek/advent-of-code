#include <stdio.h>
#include <stdlib.h>

#define LINE_BUFFER_SIZE 256

void update_top_three(int top_three[3], int sum) {
    if (sum > top_three[0]) {
        top_three[2] = top_three[1];
        top_three[1] = top_three[0];
        top_three[0] = sum;
    } else if (sum > top_three[1]) {
        top_three[2] = top_three[1];
        top_three[1] = sum;
    } else if (sum > top_three[2]) {
        top_three[2] = sum;
    }
}

int main() {
    char fname[] = "01.txt";
    FILE *f = fopen(fname, "r");
    if (f == NULL) {
        perror("Error opening file");
        return 1;
    }
    char buffer[LINE_BUFFER_SIZE];
    int current_sum = 0;
    int top_three[3] = {0};
    while (fgets(buffer, LINE_BUFFER_SIZE, f)) {
        if (buffer[0] == '\n' || buffer[0] == '\r') {
            update_top_three(top_three, current_sum);
            current_sum = 0;
        } else {
            current_sum += (int)strtol(buffer, NULL, 10);
        }
    }
    fclose(f);
    update_top_three(top_three, current_sum);
    printf("%d\n", top_three[0]);
    printf("%d\n", top_three[0] + top_three[1] + top_three[2]);
    return 0;
}
