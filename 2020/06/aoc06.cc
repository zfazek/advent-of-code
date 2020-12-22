#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <streambuf>
#include <string>
#include <vector>

#include "../utils.hh"

using namespace std;

int main() {
    ifstream file("06.txt");
    string str((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    vector<string> groups = split_string(str, "\n\n");
    int sum = 0;
    set<char> answers;
    for (const string& group : groups) {
        answers.clear();
        for (const char c : group) {
            if (c >= 'a' && c <= 'z') answers.insert(c);
        }
        sum += answers.size();
    }
    cout << sum << endl;
    sum = 0;
    map<char, int> answers2;
    for (const string& group : groups) {
        answers2.clear();
        vector<string> lines = split_string(group, "\n");
        const int group_size = lines.size();
        for (const string& line : lines) {
            for (const char c : line) {
                if (c >= 'a' && c <= 'z') answers2[c]++;
            }
        }
        for (const auto& kv : answers2) {
            if (kv.second == group_size) sum++;
        }
    }
    cout << sum << endl;
}
