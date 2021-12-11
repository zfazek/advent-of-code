#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <streambuf>
#include <string>
#include <vector>

#include "../../utils.hh"

using namespace std;

vector<set<int>> rules;
vector<vector<int>> tickets;
vector<int> yourTicket;
set<int> valids;
set<int> numbersSoFar;
vector<int> result;
bool dp[20][20];
const string FILE_NAME = "16.txt";
long start;
long n = 0;
unsigned startIndex = 6;

void foo1() {
    ifstream file(FILE_NAME);
    string str((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    vector<string> lines = split_string(str, "\n");
    set<int> numbers;
    bool nearby = false;
    long acc = 0;
    for (unsigned i = 0; i < lines.size(); i++) {
        const string line = lines[i];
        if (line.find("nearby ticket") != string::npos) {
            nearby = true;
        }
        if (line.find("-") != string::npos) {
            vector<string> tokens = split_string(line, " ");
            for (string token: tokens) {
                if (token.find("-") != string::npos) {
                    vector<string> range = split_string(token, "-");
                    int min = atoi(range[0].c_str());
                    int max = atoi(range[1].c_str());
                    for (int j = min; j <= max; j++) {
                        numbers.insert(j);
                    }
                }
            }
        }
        if (nearby && line.find(",") != string::npos) {
            vector<string> tokens = split_string(line, ",");
            bool valid = true;
            for (string token : tokens) {
                int t = atoi(token.c_str());
                if (numbers.find(t) == numbers.end()) {
                    acc += t;
                    valid = false;
                }
            }
            if (valid) {
                valids.insert(i);
            }
        }
    }
    //        printf("Valid tickets: ");
    //        for (int v : valids) {
    //            printf("%d ", v);
    //        }
    //        puts("");
    cout << acc << endl;
}

bool isCorrect(int idx, int i) {
    for (unsigned t = 0; t < tickets.size(); t++) {
        if (rules[i].find(tickets[t][idx]) == rules[i].end()) {
            return false;
        }
    }
    return true;
}

void printResult() {
    printf("Result: ");
    for (int r : result) {
        printf("%2d ", r);
    }
    puts("");
}

void solve(unsigned idx) {
    if (idx >= result.size()) {
        return;
    }
    n++;
    if (n % 10000000L == 0) {
        printf("n: %.0f M ", n / 1000000.0);
        printResult();
    }
    for (unsigned i = 0; i < result.size(); i++) {
        if (idx == 0 && i < startIndex) {
            continue;
        }
        if (numbersSoFar.find(i) != numbersSoFar.end()) {
            continue;
        }
        bool correct = dp[idx][i];
        if (correct) {
            result[idx] = i;
            if (idx == result.size() - 1) {
                puts("BINGO");
                printResult();
                long acc = 1;
                for (unsigned j = 0; j < yourTicket.size(); j++) {
                    if (result[j] < 6) {
                        acc *= yourTicket[j];
                    }
                }
                printf("n: %ld, result: %ld\n", n, acc);
                exit(0);
            }
            numbersSoFar.insert(i);
            solve(idx + 1);
            numbersSoFar.erase(i);
        }
    }
}

void foo2() {
    ifstream file(FILE_NAME);
    string str((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    vector<string> lines = split_string(str, "\n");
    unsigned yourTicketIdx = 0;
    for (unsigned i = 0; i < lines.size(); i++) {
        const string line = lines[i];
        if (line.find("your ticket") != string::npos) {
            yourTicketIdx = i + 1;
            continue;
        }
        if (line.find("-") != string::npos) {
            vector<string> tokens = split_string(line, " ");
            set<int> numbers;
            for (string token: tokens) {
                if (token.find("-") != string::npos) {
                    vector<string> range = split_string(token, "-");
                    int min = atoi(range[0].c_str());
                    int max = atoi(range[1].c_str());
                    for (int j = min; j <= max; j++) {
                        numbers.insert(j);
                    }
                }
            }
            rules.push_back(numbers);
        }
        if (line.find(",") != string::npos) {
            if (i != yourTicketIdx && valids.find(i) == valids.end()) {
                continue;
            }
            vector<string> tokens = split_string(line, ",");
            vector<int> ticket;
            for (string token : tokens) {
                int t = atoi(token.c_str());
                if (i == yourTicketIdx) {
                    yourTicket.push_back(t);
                } else {
                    ticket.push_back(t);
                }
            }
            if (i != yourTicketIdx) {
                tickets.push_back(ticket);
            }
        }
    }

    puts("Rules:");
    for (unsigned i = 0; i < rules.size(); i++) {
        const set<int> rule = rules[i];
        printf("%d: ", i);
        for (int r : rule) {
            printf("%d ", r);
        }
        puts("");
    }
    puts("");
    printf("Your ticket: ");
    for (int v : yourTicket) {
        printf("%d ", v);
    }
    puts("");
    puts("Valid tickets:");
    for (unsigned i = 0; i < tickets.size(); i++) {
        const vector<int> ticket = tickets[i];
        for (int t : ticket) {
            printf("%3d ", t);
        }
        puts("");
    }
    puts("");

    for (unsigned i = 0; i < rules.size(); i++) {
        for (unsigned j = 0; j < rules.size(); j++) {
            dp[i][j] = isCorrect(i, j);
        }
    }
    for (unsigned i = 0; i < rules.size(); i++) {
        result.push_back(-1);
    }
    solve(0);
}

int main() {
    foo1();
    foo2();
}
