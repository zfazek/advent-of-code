#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <streambuf>
#include <string>
#include <vector>

#include "../../utils.hh"

using namespace std;

class Rule {
  public:
    string name;
    int n;
};

typedef vector<Rule> Rules;

map<string, Rules> bags;

bool is_possible(const Rules& rules) {
    for (const Rule& rule : rules) {
        if (rule.name == "shiny gold") return true;
        if (is_possible(bags.at(rule.name))) return true;
    }
    return false;
}

int count_gold(const Rules& rules) {
    if (rules.empty()) return 0;
    int ret = 0;
    for (const Rule& rule : rules) {
        ret += rule.n;
        ret += rule.n * count_gold(bags.at(rule.name));
    }
    return ret;
}

int main() {
    ifstream file("07.txt");
    string str((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    vector<string> lines = split_string(str, "\n");
    for (const string& line : lines) {
        vector<string> tokens = split_string(line, " ");
        const string str_contain = " contain ";
        const size_t pos_contain = line.find(str_contain);
        const string name = tokens[0] + " " + tokens[1];
        const string str_rules = line.substr(pos_contain + str_contain.size());
        vector<string> arr_rules = split_string(str_rules, ", ");
        Rules rules;
        for (const string& rule : arr_rules) {
            if (rule.find("no other bags") != string::npos) break;
            const string rule_name =
                rule.substr(rule.find_first_of(' ') + 1, rule.find("bag") - 3);
            const int n = atoi(rule.substr(0, rule.find(" ")).c_str());
            Rule r{rule_name, n};
            rules.push_back(r);
        }
        bags[name] = rules;
    }
    int count = 0;
    for (auto& kv : bags) if (is_possible(kv.second)) count++;
    cout << count << endl;
    cout << count_gold(bags.at("shiny gold")) << endl;
}
