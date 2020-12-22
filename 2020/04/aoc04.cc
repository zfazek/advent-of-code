#include <algorithm>
#include <fstream>
#include <iostream>
#include <streambuf>
#include <string>
#include <vector>

#include "../utils.hh"

using namespace std;

int main() {
    ifstream file("04.txt");
    string str((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    vector<string> passports = split_string(str, "\n\n");
    int sum = count_if(passports.begin(), passports.end(),
            [](const string& passport){
            size_t n = count(passport.begin(), passport.end(), ':');
            if (passport.find("cid") != string::npos) n--;
            return n == 7;});
    cout << sum << endl;
}
