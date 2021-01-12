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
    ifstream myfile("03.txt");
    vector<string> lines;
    if (!myfile.is_open()) {
        cout << "Unable to open file"; 
        return 1;
    }
    while (getline(myfile, line)) {
        // cerr << line << endl;
        lines.push_back(line);
    }
    long result = 0;
    int pos = 0;
    for (const string &line : lines) {
        const int length = line.size();
        if (line[pos % length] == '#') ++result;
        pos += 3;
    }
    cout << result << endl;
    result = 1;
    int row = 1;
    vector<int> jumps{1, 3, 5, 7, 1};
    for (const int jump : jumps) {
        int pos = 0;
        int counter = 0;
        for (unsigned i = 0; i < lines.size(); ++i) {
            if (jump == 1 && row == 2 && i % 2 == 1) continue;
            const string line = lines[i];
            // cerr << line << endl;
            const int length = line.size();
            if (line[pos % length] == '#') counter++;
            pos += jump;
        }
        myfile.close();
        result *= counter;
        // cerr << "jump: " << jump << " counter: " << counter << endl;
        if (jump == 1) row = 2;
    }
    cout << result << endl;
    return 0;
}
