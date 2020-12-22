#include <algorithm>
#include <fstream>
#include <iostream>
#include <set>
#include <string>

using namespace std;

int main () {
    set<int> numbers;
    string line;
    ifstream myfile("adventofcode_202001.txt");
    if (!myfile.is_open()) {
        cout << "Unable to open file"; 
        return 1;
    }
    while (getline(myfile, line)) {
        // cout << line << '\n';
        const int n = atoi(line.c_str());
        numbers.insert(n);
    }
    myfile.close();
    for (const int n : numbers) {
        if (numbers.find(2020 - n) != numbers.end()) {
            cout << n * (2020 - n) << endl;
            break;
        }
    }
    for (const int n : numbers) {
        for (const int m : numbers) {
            if (numbers.find(2020 - n - m) != numbers.end()) {
                cout << n * m * (2020 - n - m) << endl;
                return 0;
            }
        }
    }
    return 0;
}
