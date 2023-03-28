#include <algorithm>
#include <fstream>
#include <iostream>
#include <streambuf>
#include <string>
#include <vector>

#include "../../utils.hh"

using namespace std;

int main() {
    ifstream file("01.txt");
    string str((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    vector<string> blocks = split_string(str, "\n\n");
    vector<long> sums;
    for (auto block : blocks) {
        vector<string> line = split_string(block, "\n");
        long sum = 0;
        for (auto p : line) {
            sum += stoi(p);
        }
        sums.push_back(sum);
    }
    sort(sums.begin(), sums.end(), greater<long>());
    cout << sums[0] << endl;
    cout << sums[0] + sums[1] + sums[2] << endl;
}
