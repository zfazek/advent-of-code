#pragma once

#include <stdlib.h>

typedef struct BTree {
    int v;
    struct BTree *left;
    struct BTree *right;
} BTree;

BTree *btree_create_new_node_int(int v) {
    BTree *node = (BTree *)malloc(sizeof(BTree));
    node->v = v;
    node->left = NULL;
    node->right = NULL;
    return node;
}

void btree_insert_int(BTree **head, int v) {
    if (*head == NULL) {
        *head = btree_create_new_node_int(v);
        return;
    }
    BTree *node = *head;
    while (node) {
        if (v == node->v) {
            return;
        } else if (v < node->v) {
            if (node->right == NULL) {
                node->right = btree_create_new_node_int(v);
                return;
            } else {
                node = node->right;
            }
        } else {
            if (node->left == NULL) {
                node->left = btree_create_new_node_int(v);
                return;
            } else {
                node = node->left;
            }
        }
    }
}

int btree_contains_int(BTree *head, int v) {
    BTree *node = head;
    while (node) {
        if (v == node->v) {
            return 1;
        } else if (v < node->v) {
            node = node->right;
        } else {
            node = node->left;
        }
    }
    return 0;
}
