#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <set>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

int main () {
    string line;
    ifstream myfile("02.txt");
    if (!myfile.is_open()) {
        cout << "Unable to open file"; 
        return 1;
    }
    int counter1 = 0;
    int counter2 = 0;
    while (getline(myfile, line)) {
        istringstream iss(line);
        vector<string> tokens(istream_iterator<string>{iss},
                istream_iterator<string>());
        const string range = tokens[0];
        const string password = tokens[2];
        const char ch = tokens[1][0];
        const int min = atoi(range.substr(0, range.find_first_of('-')).c_str());
        const int max = atoi(range.substr(range.find_first_of('-') + 1).c_str());
        // cerr << min << " " << max << " " << ch << " " << password << endl;
        int sum1 = 0;
        for (const char c : password) {
            if (c == ch) sum1++;
        }
        if (sum1 >= min && sum1 <= max) counter1++;
        int sum2 = 0;
        if (password[min - 1] == ch) sum2++;
        if (password[max - 1] == ch) sum2++;
        if (sum2 == 1) counter2++;
    }
    myfile.close();
    cout << "result 1: " << counter1 << endl;
    cout << "result 2: " << counter2 << endl;
    return 0;
}
