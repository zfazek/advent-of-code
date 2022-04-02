#pragma once

#include <stdlib.h>

typedef struct BTree {
    int k;
    int v;
    struct BTree *left;
    struct BTree *right;
} BTree;

BTree *btree_create_new_node_int(int k, int v) {
    BTree *node = (BTree *)malloc(sizeof(BTree));
    node->k = k;
    node->v = v;
    node->left = NULL;
    node->right = NULL;
    return node;
}

void btree_insert_int(BTree **head, int k, int v) {
    if (*head == NULL) {
        *head = btree_create_new_node_int(k, v);
        return;
    }
    BTree *node = *head;
    while (node) {
        if (k == node->k) {
            return;
        } else if (k < node->k) {
            if (node->right == NULL) {
                node->right = btree_create_new_node_int(k, v);
                return;
            } else {
                node = node->right;
            }
        } else {
            if (node->left == NULL) {
                node->left = btree_create_new_node_int(k, v);
                return;
            } else {
                node = node->left;
            }
        }
    }
}

BTree *btree_contains_int(BTree *head, int k) {
    BTree *node = head;
    while (node) {
        if (k == node->k) {
            return node;
        } else if (k < node->k) {
            node = node->right;
        } else {
            node = node->left;
        }
    }
    return NULL;
}
