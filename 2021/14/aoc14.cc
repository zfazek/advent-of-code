#include "../../utils.hh"

#include <chrono>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <streambuf>
#include <string>
#include <vector>

using namespace std;
using namespace std::chrono;

int main() {
    ifstream file("14.txt");
    string input((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    long start = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    vector<string> parts = split_string(input, "\n\n");
    string polymer_template = parts[0];
    vector<string> input_rules = split_string(parts[1], "\n");
    map<string, string> rules;
    for (const string& rule : input_rules) {
        vector<string> tokens = split_string(rule, " -> ");
        rules[tokens[0]] = tokens[1];
    }
    map<string, long> t;
    map<string, long> tt;
    for (int i = 1; i < polymer_template.size(); ++i) {
        const string pair = polymer_template.substr(i - 1, 2);
        t[pair]++;
    }
    for (int n = 0; n < 40; ++n) {
        tt = t;
        for (const auto& kv : t) {
            const string k = kv.first;
            const string c = rules.at(k);
            const string p1 = k[0] + c;
            const string p2 = c + k[1];
            tt[k] -= kv.second;
            tt[p1] += kv.second;
            tt[p2] += kv.second;
        }
        t = tt;
    }
    map<char, long> counter;
    counter[polymer_template[0]]++;
    for (auto& kv : t) {
        counter[kv.first[1]] += kv.second;
    }
    long max = 0;
    long min = 9223372036854775807L;
    for (const auto& kv : counter) {
        const long v = kv.second;
        min = std::min(min, v);
        max = std::max(max, v);
    }
    cout << max - min << endl;
    long end = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    cout << end - start << " ms" << endl;
}
