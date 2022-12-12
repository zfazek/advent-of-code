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

int sx, sy, ex, ey;
vector<vector<char>> arr;
map<int, int> best;
priority_queue<Elem> pq;
int ans;

void print() {
    for (unsigned i = 0; i < arr.size(); ++i) {
        for (unsigned j = 0; j < arr[0].size(); ++j) {
            printf("%c", arr[i][j]);
        }
        puts("");
    }
    puts("");
}

void init() {
    for (unsigned i = 0; i < arr.size(); ++i) {
        for (unsigned j = 0; j < arr[0].size(); ++j) {
            const int idx = i * arr[0].size() + j;
            best[idx] = 999999999;
        }
    }
}

int foo(Elem e) {
    init();
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
                    int x = e.x + dx;
                    int y = e.y + dy;
                    if (x >= 0 && x < arr[0].size() && y >= 0 && y < arr.size()) {
                        if (arr[y][x] <= arr[e.y][e.x] + 1) {
                            Elem e1(e.cost + 1, x, y);
                            pq.push(e1);
                        }
                    }
                }
            }
        }
    }
    return best.at(ey * arr[0].size() + ex);
}

void one() {
    ans = arr.size() * arr[0].size();
    Elem e(0, sx, sy);
    cout << foo(e) << endl;
}

void two() {
    ans = arr.size() * arr[0].size();
    for (unsigned i = 0; i < arr.size(); ++i) {
        for (unsigned j = 0; j < arr[0].size(); ++j) {
            if (arr[i][j] == 'a') {
                sx = j;
                sy = i;
                Elem e(0, sx, sy);
                int r = foo(e);
                if (r < ans) {
                    ans = r;
                }
            }
        }
    }
    cout << ans << endl;
}

int main() {
    ifstream file("12.txt");
    string input((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    long start = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    vector<string> lines = split_string(input, "\n");
    for (unsigned i = 0; i < lines.size(); ++i) {
        vector<char> row(lines[i].size());
        for (unsigned j = 0; j < lines[i].size(); ++j) {
            row[j] = lines[i][j];
            if (row[j] == 'S') {
                sx = j;
                sy = i;
                row[j] = 'a';
            } else if (row[j] == 'E') {
                ex = j;
                ey = i;
                row[j] = 'z';
            }
        }
        arr.push_back(row);
    }
    print();
    one();
    two();
    long end = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    cout << end - start << " ms" << endl;
}
