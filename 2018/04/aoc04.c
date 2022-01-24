#include "../../lib/utils_file.h"
#include "../../lib/utils_ds.h"
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 50

// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up
// [1518-11-01 00:30] falls asleep
// [1518-11-01 00:55] wakes up
// [1518-11-01 23:58] Guard #99 begins shift
// [1518-11-02 00:40] falls asleep
// [1518-11-02 00:50] wakes up
// [1518-11-03 00:05] Guard #10 begins shift
// [1518-11-03 00:24] falls asleep
// [1518-11-03 00:29] wakes up
// [1518-11-04 00:02] Guard #99 begins shift
// [1518-11-04 00:36] falls asleep
// [1518-11-04 00:46] wakes up
// [1518-11-05 00:03] Guard #99 begins shift
// [1518-11-05 00:45] falls asleep
// [1518-11-05 00:55] wakes up

typedef struct Guard {
    int id;
    int minutes[60];
    struct Guard *next;
} Guard;

Guard *head = NULL;
Guard *current = NULL;

Guard *insert(const int id) {
    Guard *node = (Guard *)malloc(sizeof(Guard));
    node->id = id;
    memset(node->minutes, 0, 60 * sizeof(int));
    node->next = NULL;
    if (current) {
        current->next = node;
    }
    current = node;
    if (head == NULL) {
        head = node;
    }
    return current;
}

Guard *find(const int id) {
    if (head == NULL) {
        return NULL;
    }
    Guard *node = head;
    while (node) {
        if (node->id == id) {
            return node;
        }
        node = node->next;
    }
    return NULL;
}

void clear() {
    Guard *node = head;
    while (node) {
        Guard *next = node->next;
        free(node);
        node = next;
    }
    head = NULL;
    current = NULL;
}

void one(char lines[][BUFFER_SIZE], const int n_lines) {
    int id, start, end;
    for (int l = 0; l < n_lines; ++l) {
        char *curr = strdup(lines[l]);
        char *line = curr;
        char *date = strsep(&line, "]");
        if (line[1] == 'G') {
            id = atoi(line + 8);
        } else if (line[1] == 'f') {
            start = atoi(date + 15);
        } else if (line[1] == 'w') {
            end = atoi(date + 15);
            Guard *node = find(id);
            if (node == NULL) {
                node = insert(id);
            }
            for (int i = start; i < end; ++i) {
                ++node->minutes[i];
            }
        }
        free(curr);
    }
    int max = 0;
    int max_id;
    Guard *node = head;
    while (node) {
        int sum = 0;
        for (int i = 0; i < 60; ++i) {
            sum += node->minutes[i];
        }
        if (sum > max) {
            max = sum;
            max_id = node->id;
        }
        node = node->next;
    }
    node = find(max_id);
    max = 0;
    int max_day;
    for (int i = 0; i < 60; ++i) {
        if (node->minutes[i] > max) {
            max_day = i;
            max = node->minutes[i];
        }
    }
    printf("%d\n", node->id * max_day);
    clear();
}

void two(char lines[][BUFFER_SIZE], const int n_lines) {
    int id, start, end;
    for (int l = 0; l < n_lines; ++l) {
        char *curr = strdup(lines[l]);
        char *line = curr;
        char *date = strsep(&line, "]");
        if (line[1] == 'G') {
            id = atoi(line + 8);
        } else if (line[1] == 'f') {
            start = atoi(date + 15);
        } else if (line[1] == 'w') {
            end = atoi(date + 15);
            Guard *node = find(id);
            if (node == NULL) {
                node = insert(id);
            }
            for (int i = start; i < end; ++i) {
                ++node->minutes[i];
            }
        }
        free(curr);
    }
    int max = 0;
    int max_id;
    int max_day;
    Guard *node = head;
    while (node) {
        for (int i = 0; i < 60; ++i) {
            if (node->minutes[i] > max) {
                max_day = i;
                max = node->minutes[i];
                max_id = node->id;
            }
        }
        node = node->next;
    }
    printf("%d\n", max_id * max_day);
    clear();
}

int main() {
    char fname[] = "04.txt";
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
    qsort(lines, n_lines, sizeof lines[0], compare_string);
    one(lines, n_lines);
    two(lines, n_lines);
}
