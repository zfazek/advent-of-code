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

void hashmap_insert_int(Hash *hash, int k, int v) {
    int hash_id = hash_code(k);
    btree_insert_int(&hash[hash_id].node, k, v);
}

BTree *hashmap_contains_int(Hash *hash, int k) {
    int hash_id = hash_code(k);
    return btree_contains_int(hash[hash_id].node, k);
}

