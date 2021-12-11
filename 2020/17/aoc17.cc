#include <fstream>
#include <iostream>
#include <streambuf>
#include <string>
#include <vector>
#include <cstring>

#include "../../utils.hh"

using namespace std;

static constexpr size_t NUMBER_OF_CYCLES = 6;

void print(const bool arr[], const size_t N) {
    const size_t NN = N * N;
    for (size_t z = 0; z < N; ++z) {
        bool found = false;
        string str;
        for (size_t y = 0; y < N; ++y) {
            for (size_t x = 0; x < N; ++x) {
                const size_t pos = z * NN + y * N + x;
                if (arr[pos]) {
                    str += "#";
                    found = true;
                }
                else {
                    str += ".";
                }
            }
            str += "\n";
        }
        if (found) {
        cout << "z: " << z << endl << str << endl << endl;
        }
    }
}

int get_number_of_neighbors(const bool arr[], const size_t idx, const size_t N) {
    int acc = 0;
    const size_t NN = N * N;
    const size_t NNN = N * N * N;
    const size_t w = idx / NNN;
    const size_t z = (idx - w * NNN) / NN;
    const size_t y = (idx - w * NNN - z * NN) / N;
    const size_t x = idx - w * NNN - z * NN - y * N;
    //cout <<idx<<" "<<z<<" "<<y<<" "<<x<<" ";
    for (int h = -1; h <= 1; ++h) {
        for (int i = -1; i <= 1; ++i) {
            for (int j = -1; j <= 1; ++j) {
                for (int k = -1; k <= 1; ++k) {
                    if (h == 0 && i == 0 && j == 0 && k == 0) continue;
                    if (w + h >= 0 && w + h < N &&
                            z + i >= 0 && z + i < N &&
                            y + j >= 0 && y + j < N &&
                            x + k >= 0 && x + k < N) {
                        const size_t pos = (w + h) * NNN + (z + i) * NN + (y + j) * N + x + k;
                        if (arr[pos]) {
                            ++acc;
                        }
                    }
                }
            }
        }
    }
    return acc;
}

size_t get_number_of_active(const bool arr[], const size_t N) {
    size_t acc = 0;
    static const size_t NNNN = N * N * N * N;
    for (size_t i = 0; i < NNNN; ++i) {
        if (arr[i]) {
            ++acc;
        }
    }
    return acc;
}

int main() {
    ifstream file("17.txt");
    string str((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    vector<string> lines = split_string(str, "\n");
    const size_t N = lines.size() + NUMBER_OF_CYCLES * 2;
    const size_t NN = N * N;
    const size_t NNN = N * N * N;
    const size_t NNNN = N * N * N * N;
    const size_t w = N / 2;
    const size_t z = N / 2;
    bool arr[NNNN];
    bool temp_arr[NNNN];
    for (size_t i = 0; i < NNNN; ++i) {
        arr[i] = false;
    }
    for (size_t i = 0; i < lines.size(); ++i) {
        for (size_t j = 0; j < lines[i].size(); ++j) {
            const char c = lines[i][j];
            if (c == '#') {
                const int pos = w * NNN + z * NN + (i + NUMBER_OF_CYCLES) * N + j + NUMBER_OF_CYCLES;
                arr[pos] = true;
            }
        }
    }
    //print(arr, N);
    cout << get_number_of_active(arr, N)  << endl;
    for (size_t c = 0; c < NUMBER_OF_CYCLES; ++c) {
        for (size_t i = 0; i < NNNN; ++i) {
            const int n_neighbors = get_number_of_neighbors(arr, i, N);
            //cout<<arr[i]<<endl;
            if (arr[i]) {
                if (n_neighbors == 2 || n_neighbors == 3) {
                    temp_arr[i] = true;
                } else {
                    temp_arr[i] = false;
                }
            } else {
                if (n_neighbors == 3) {
                    temp_arr[i] = true;
                } else {
                    temp_arr[i] = false;
                }
            }
        }
        memcpy(arr, temp_arr, NNNN * sizeof(bool));
        //print(arr, N);
        cout << get_number_of_active(arr, N)  << endl;
    }
}
