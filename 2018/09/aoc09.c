#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define PLAYERS 486
#define MAX 7083300

typedef struct Node {
    int v;
    struct Node *prev;
    struct Node *next;
} Node;

Node *head;
Node *current;

long scores[PLAYERS];

int main() {
    Node *node = (Node *)malloc(sizeof(Node));
    node->v = 0;
    node->prev = node;
    node->next = node;
    head = node;
    current = node;
    for (int i = 1; i <= MAX; ++i) {
        const int player = i % PLAYERS;
        if (i % 23 == 0) {
            scores[player] += i;
            for (int j = 0; j < 7; ++j) {
                current = current->prev;
            }
            scores[player] += current->v;
            Node *tmp = current;
            current->prev->next = current->next;
            current->next->prev = current->prev;
            current = current->next;
            free(tmp);
        } else {
            current = current->next;
            Node *next = current->next;
            Node *new = (Node *)malloc(sizeof(Node));
            current->next = new;
            new->next = next;
            new->prev = current;
            new->v = i;
            current = new;
            current->next->prev = current;
        }
    }
    long max = 0;
    for (int i = 0; i < PLAYERS; ++i) {
        if (scores[i] > max) {
            max = scores[i];
        }
    }
    current = head->next;
    while (current != head) {
        current = current->next;
        free(current->prev);
    }
    free(head);
    printf("%lu\n", max);
}
