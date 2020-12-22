#include <algorithm>
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <vector>

using namespace std;

int get_score(const string line) {
    int s = 0;
    int d = 1;
    for (int i = line.size() - 1; i >= 0; --i) {
        const char c = line[i];
        if (c == 'B' || c == 'R') {
            s += d;
        }
        d *= 2;
    }
    return s;
}

int main () {
    set<int> ids;
    string line;
    ifstream myfile("adventofcode_202005.txt");
    if (!myfile.is_open()) {
        cout << "Unable to open file"; 
        return 1;
    }
    int max = 0;
    string pass;
    while (getline(myfile, line)) {
        // cout << line << '\n';
        const int score = get_score(line);
        ids.insert(score);
        if (score > max) {
            max = score;
            pass = line;
        }
    }
    myfile.close();
    cout << get_score(pass) << endl;
    for (int i = 1; i < 1023; ++i) {
        if (ids.find(i - 1) != ids.end() &&
                ids.find(i) == ids.end() &&
                ids.find(i + 1) != ids.end()) {
            cout << i << endl;
        }
    }
    return 0;
}
