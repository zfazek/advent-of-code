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

struct Instruction {
    char cmd;
    int v = 0;
    bool visited = false;

    friend ostream& operator<<(ostream& out, const Instruction& i) {
        out << "cmd: " << i.cmd << ", value: " << i.v << ", visited: " <<
            (i.visited ? "true" : "false");
        return out;
    }
};

typedef vector<Instruction> Instructions;

void swap(Instructions& instructions, const size_t i) {
    if (instructions[i].cmd == 'n') {
        instructions[i].cmd = 'j';
    } else {
        instructions[i].cmd = 'n';
    }
}

int main() {
    ifstream file("08.txt");
    string str((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    vector<string> lines = split_string(str, "\n");
    Instructions instructions;
    for (const string& line : lines) {
        Instruction instruction;
        instruction.cmd = line[0];
        const char sign = line[4];
        if (instruction.cmd != 'n') {
            instruction.v = atoi(line.substr(5).c_str());
            if (sign == '-') {
                instruction.v = -instruction.v;
            }
        }
        instructions.push_back(instruction);
    }
    for (size_t i = 0; i < instructions.size(); ++i) {
        if (instructions[i].cmd == 'a') {
            continue;
        }
        swap(instructions, i);
        int acc = 0;
        int p = 0;
        for (auto& instruction : instructions) {
            instruction.visited = false;
        }
        while (!instructions[p].visited && p >= 0 && p < instructions.size()) {
            if (instructions[p].cmd == 'a') {
                acc += instructions[p].v;
                instructions[p].visited = true;
                p++;
            } else if (instructions[p].cmd == 'j') {
                instructions[p].visited = true;
                p += instructions[p].v;
            } else {
                instructions[p].visited = true;
                p++;
            }
        }
        if (p == instructions.size()) {
            cout << acc << endl;
            break;
        }
        swap(instructions, i);
    }
}
