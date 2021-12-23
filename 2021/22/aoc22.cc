#include "../../utils.hh"

#include <chrono>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>
#include <queue>

using namespace std;
using namespace std::chrono;

// on x=10..12,y=10..12,z=10..12

constexpr int N = 50;
constexpr int SIZE = 2 * N + 1;
int arr[SIZE][SIZE][SIZE];

int main() {
    ifstream file("22.txt");
    string input((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    long start = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    vector<string> lines = split_string(input, "\n");
    for (string line : lines) {
        cout << line << endl;
        vector<string> tokens = split_string(line, " ");
        string cmd = tokens[0];
        vector<string> coords = split_string(tokens[1], ",");
        int x1 = stoi(split_string(split_string(coords[0], "=")[1], "..")[0]);
        int x2 = stoi(split_string(split_string(coords[0], "=")[1], "..")[1]);
        int y1 = stoi(split_string(split_string(coords[1], "=")[1], "..")[0]);
        int y2 = stoi(split_string(split_string(coords[1], "=")[1], "..")[1]);
        int z1 = stoi(split_string(split_string(coords[2], "=")[1], "..")[0]);
        int z2 = stoi(split_string(split_string(coords[2], "=")[1], "..")[1]);
        int v = 0;
        if (cmd == "on") {
            v = 1;
        }
//        for (int x = x1 + N; x <= x2 + N; x++) {
//            for (int y = y1 + N; y + N <= y2; y++) {
//                for (int z = z1 + N; z + N <= z2; z++) {
//                    if (x > 0 && x < SIZE && y > 0 && y < SIZE && z > 0 && z < SIZE) {
//                        arr[x][y][z] = v;
//                    }
//                }
//            }
//        }
        for (int i = -N; i <= N; i++) {
            for (int j = -N; j <= N; j++) {
                for (int k = -N; k <= N; k++) {
                    int i1 = i + N;
                    int j1 = j + N;
                    int k1 = k + N;
                    if (i >= x1 && i <= x2 && j >= y1 && j <= y2 && k >= z1 && k <= z2) {
                        arr[i1][j1][k1] = v;
                    }
                }
            }
        }
    }
    long n = 0;
    for (int i = -N; i <= N; i++) {
        for (int j = -N; j <= N; j++) {
            for (int k = -N; k <= N; k++) {
                int i1 = i + N;
                int j1 = j + N;
                int k1 = k + N;
                if (arr[i1][j1][k1] == 1) {
                    n++;
                }
            }
        }
    }
    long end = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    cout << n << endl;
    cout << end - start << " ms" << endl;
}
