#include <fstream>
#include <iostream>
#include <set>
#include <vector>
#include <cstdlib>

using namespace std;

void one(const vector<int>& input) {
    int n = 0;
    for (const int num : input) {
        n += num;
    }
    cout << n << endl;
}

void two(const vector<int>& input) {
    int n = 0;
    set<int> seen;
    while (true) {
        for (const int num : input) {
            n += num;
            if (seen.find(n) != seen.end()) {
                cout << n << endl;
                exit(0);
            } else {
                seen.insert(n);
            }
        }
    }
}

int main() {
    ifstream myfile("01.txt");
    if (!myfile.is_open()) {
        cout << "Unable to open file\n";
        return 1;
    }
    vector<int> input;
    string line;
    while (getline(myfile, line)) {
        const int n = atoi(line.c_str());
        input.push_back(n);
    }
    myfile.close();
    one(input);
    two(input);
}
