#include <stdlib.h>

int compare_int(const void *a, const void *b) {
    return *(int*)a - *(int*)b;
}

int binary_search(int arr[], int l, int r, int x) {
    if (r >= l) {
        int mid = l + (r - l) / 2;
        if (arr[mid] == x)
            return mid;
        if (arr[mid] > x)
            return binary_search(arr, l, mid - 1, x);
        return binary_search(arr, mid + 1, r, x);
    }
    return -1;
}

void insertion_sort(int *A, int size) {
    int i, key, j;
    for (i = 1; i < size; i++) {
        key = A[i];
        j = i - 1;
        while (j >= 0 && A[j] > key) {
            A[j + 1] = A[j];
            j = j - 1;
        }
        A[j + 1] = key;
    }
}

int index_arr(const int *seen, const int seen_idx, const int n) {
    for (int i = 0; i < seen_idx; i++) {
        if (seen[i] == n) {
            return i;
        }
    }
    return -1;
}

