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


int main() {
    ifstream file("20.txt");
    string input((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    long start = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    vector<string> lines = split_string(input, "\n");
    string algorithm = lines.at(0);
    int T = 50;
    int border = 2 * T;
    int arrIdx = 0;
    int size = max(lines.size() - 2, lines.at(2).length()) + 2 * border;
    char arr[size][size][2];
    for (int i = 0; i < size; ++i) {
        for (int j = 0; j < size; ++j) {
            arr[i][j][arrIdx] = '.';
        }
    }

    for (int i = 2; i < lines.size(); ++i) {
        for (int j = 0; j < lines.at(i).length(); ++j) {
            arr[i - 2 + border][j + border][arrIdx] = lines.at(i).at(j);
        }
    }
    for (int t = 0; t < T; ++t) {
        for (int i = 0; i < size; ++i) {
            for (int j = 0; j < size; ++j) {
                stringstream ss;
                char c;
                for (int di = -1; di < 2; di++) {
                    for (int dj = -1; dj < 2; dj++) {
                        if (i + di < 0 || i + di >= size || j + dj < 0 || j + dj >= size) {
                            c = arr[0][0][arrIdx];
                        } else {
                            c = arr[i + di][j + dj][arrIdx];
                        }
                        if (c == '.') {
                            ss << "0";
                        } else {
                            ss << "1";
                        }
                    }
                }
                int idx = stoi(ss.str(), 0, 2);
                arr[i][j][1 - arrIdx] = algorithm.at(idx);
            }
        }
        arrIdx = 1 - arrIdx;
    }
    long n = 0;
    for (int i = 0; i < size; ++i) {
        for (int j = 0; j < size; ++j) {
            if (arr[i][j][arrIdx] == '#') {
                ++n;
            }
        }
    }
    long end = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    cout << n << endl;
    cout << end - start << " ms" << endl;
}
