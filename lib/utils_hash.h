#pragma once

#include "utils_btree.h"
#include <stdlib.h>
#include <stdio.h>

typedef struct Hash {
    BTree *node;
} Hash;

#define HASH_SIZE 1000

int hash_code(int v) {
    return abs(v % HASH_SIZE);
}

void hash_insert(Hash *hash, int v) {
    int hash_id = hash_code(v);
    btree_insert_int(&hash[hash_id].node, v);
}

int hash_contains(Hash *hash, int v) {
    int hash_id = hash_code(v);
    return btree_contains_int(hash[hash_id].node, v);
}

