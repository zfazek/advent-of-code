#include <iostream>
#include <sstream>
#include <iomanip>
#include <openssl/md5.h>

using namespace std;

void one() {
    string key = "wtnhxymk";
    int n = 0;
    unsigned char result[MD5_DIGEST_LENGTH];
    for (int i = 0; i < 8; ++i) {
        while (true) {
            ++n;
            const string input = key + to_string(n);
            MD5((unsigned char*)input.c_str(), input.size(), result);
            std::ostringstream hash;
            hash << std::hex << std::setfill('0');
            for (unsigned char c: result) {
                hash << std::setw(2) << (long)c;
            }
            if (hash.str().substr(0, 5) == "00000") {
                cout << hash.str()[5] << flush;
                break;
            }
        }
    }
    cout << endl;
}

void two() {
    unsigned char result[MD5_DIGEST_LENGTH];
    string key = "wtnhxymk";
    char password[8] = {0};
    int n = 0;
    for (int i = 0; i < 8; ++i) {
        while (true) {
            ++n;
            const string input = key + to_string(n);
            MD5((unsigned char*)input.c_str(), input.size(), result);
            std::ostringstream hash;
            hash << std::hex << std::setfill('0');
            for (unsigned char c: result) {
                hash << std::setw(2) << (long long)c;
            }
            if (hash.str().substr(0, 5) == "00000") {
                const char c = hash.str()[5];
                if (c >= '0' && c <= '7') {
                    if (password[c - '0'] == 0) {
                        password[c - '0'] = hash.str()[6];
                        for (const char c : password) {
                            if (c == 0) {
                                cout << " ";
                            } else {
                                cout << c;
                            }
                        }
                        cout << endl;
                        break;
                    }
                }
            }
        }
    }
}

int main() {
    one();
    two();
}