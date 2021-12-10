#include <algorithm>
#include <chrono>
#include <fstream>
#include <iostream>
#include <stack>
#include <vector>

using namespace std;
using namespace std::chrono;

int main() {
    string line;
    ifstream myfile("2021/10/10.txt");
    if (!myfile.is_open()) {
        cout << "Unable to open file\n";
        return 1;
    }
    auto start = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    vector<string> input;
    while (getline(myfile, line)) {
        input.push_back(line);
    }
    myfile.close();
    long sum1 = 0;
    vector<long> totalScores;
    string opens = "([{<";
    string closes = ")]}>";
    int scores[] = {3, 57, 1197, 25137};
    for (string line : input) {
        long s2 = 0;
        stack<char> stack;
        bool incomplete = true;
        for (int i = 0; i < line.size(); ++i) {
            const char c = line[i];
            int idx = opens.find(c);
            if (idx != std::string::npos) {
                stack.push(c);
            } else {
                idx = closes.find(c);
                char s = stack.top();
                if (opens.find(s) == idx) {
                    stack.pop();
                } else {
                    sum1 += scores[idx];
                    incomplete = false;
                    break;
                }
            }
        }
        if (incomplete) {
            vector<char> arr;
            while (!stack.empty()) {
                arr.push_back(stack.top());
                stack.pop();
            }
            for (char c : arr) {
                const int s = opens.find(c) + 1;
                s2 = s2 * 5 + s;
            }
            totalScores.push_back(s2);
        }
    }
    cout << sum1 << endl;
    std::sort(totalScores.begin(), totalScores.end());
    cout << totalScores[totalScores.size() / 2] << endl;
    auto end = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    cout << end - start << " ms\n";

}
