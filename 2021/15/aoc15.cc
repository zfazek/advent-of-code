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

struct Elem {
    int cost;
    int x;
    int y;

    Elem(const int cost_, const int x_, const int y_) : cost{cost_}, x{x_}, y{y_} {}

    friend bool operator<(const Elem& lhs, const Elem& rhs) {
        return lhs.cost > rhs.cost;
    }

    friend ostream& operator<<(ostream& os, const Elem& e) {
        return os << e.x << ", " << e.y << ": " << e.cost << "\n";
    }
};

static int len;
vector<vector<int>> arr;
map<int, int> best;
priority_queue<Elem> pq;

void print() {
    for (int i = 0; i < len; ++i) {
        for (int j = 0; j < len; ++j) {
            printf("%4d", arr[i][j]);
        }
        puts("");
    }
    puts("");
}

int main() {
    ifstream file("15.txt");
    string input((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    long start = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    vector<string> lines = split_string(input, "\n");
    len = lines.size();
    constexpr int N = 5;
    for (int n = 0; n < N; ++n) {
        for (int i = 0; i < len; ++i) {
            vector<int> row(N * len);
            for (int m = 0; m < N; ++m) {
                for (int j = 0; j < len; ++j) {
                    int v = lines[i][j] - '0' + n + m;
                    if (v > 9) {
                        v -= 9;
                    }
                    row[m * len + j] = v;
                }
            }
            arr.push_back(row);
        }
    }
    for (int i = 0; i < arr.size(); ++i) {
        for (int j = 0; j < arr[0].size(); ++j) {
            const int idx = i * arr[0].size() + j;
            best[idx] = 999999999;
        }
    }
    arr[0][0] = 0;
    Elem e(0, 0, 0);
    pq.push(e);
    while (!pq.empty()) {
        Elem e = pq.top();
        pq.pop();
        if (e.cost < best.at(e.y * arr[0].size() + e.x)) {
            best[e.y * arr[0].size() + e.x] = e.cost;
            for (int dy = -1; dy < 2; ++dy) {
                for (int dx = -1; dx < 2; ++dx) {
                    if (dx == 0 && dy == 0) continue;
                    if (abs(dx) + abs(dy) == 2) continue;
                    const int x = e.x + dx;
                    const int y = e.y + dy;
                    if (x >= 0 && x < arr[0].size() && y >= 0 && y < arr.size()) {
                        Elem e1(e.cost + arr[y][x], x, y);
                        pq.push(e1);
                    }
                }
            }
        }
    }
    cout << best.at(arr.size() * arr[0].size() - 1) << endl;
    long end = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    cout << end - start << " ms" << endl;
}
