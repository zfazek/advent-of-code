#include "../../utils.hh"

#include <chrono>
#include <string>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <streambuf>
#include <vector>

using namespace std;
using namespace std::chrono;

int sizeX, sizeY;
bool arr[2000][2000];

void print() {
    for (int i = 0; i < sizeY; i++) {
        for (int j = 0; j < sizeX; j++) {
            char c = arr[i][j] ? '#' : '.';
            printf("%c", c);
        }
        puts("");
    }
    puts("");
}

int getNumberOfDots() {
    int sum = 0;
    for (int i = 0; i < sizeY; i++) {
        for (int j = 0; j < sizeX; j++) {
            if (arr[i][j]) {
                sum++;
            }
        }
    }
    return sum;
}

void foldY(int idx) {
    for (int i = 0; i <= idx; i++) {
        for (int j = 0; j < sizeX; j++) {
            if (i == idx) {
                arr[i][j] = false;
            } else {
                int from = 2 * idx - i;
                if (from < sizeY) {
                    arr[i][j] |= arr[from][j];
                    arr[from][j] = false;
                }
            }
        }
    }
    sizeY = idx;
}

void foldX(int idx) {
    for (int i = 0; i < sizeY; i++) {
        for (int j = 0; j <= idx; j++) {
            if (j == idx) {
                arr[i][j] = false;
            } else {
                int from = 2 * idx - j;
                if (from < sizeX) {
                    arr[i][j] |= arr[i][from];
                    arr[i][from] = false;
                }
            }
        }
    }
    sizeX = idx;
}

void fold(const string &dir, int idx) {
    if (dir == "x") {
        foldX(idx);
    } else {
        foldY(idx);
    }
}

int main() {
    ifstream file("13.txt");
    string input((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    long start = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    vector<string> parts = split_string(input, "\n\n");
    vector<string> dots = split_string(parts[0], "\n");
    vector<string> cmds = split_string(parts[1], "\n");
    sizeX = -1;
    sizeY = -1;
    for (const string &dot : dots) {
        vector<string> line = split_string(dot, ",");
        int x = stoi(line[0]);
        int y = stoi(line[1]);
        sizeX = std::max(x, sizeX);
        sizeY = std::max(y, sizeY);
    }

    sizeX++;
    sizeY++;
    for (string dot : dots) {
        vector<string> line = split_string(dot, ",");
        int x = stoi(line[0]);
        int y = stoi(line[1]);
        arr[y][x] = true;
    }
    for (string line : cmds) {
        vector<string> tokens = split_string(line, " ");
        vector<string> cmd = split_string(tokens[2], "=");
        string dir = cmd[0];
        int idx = stoi(cmd[1]);
        fold(dir, idx);
        printf("%d\n", getNumberOfDots());
    }
    long end = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    print();
    cout << end - start << " ms" << endl;
}
