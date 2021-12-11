#pragma once

#include <string>
#include <vector>

std::vector<std::string> split_string(const std::string& str,
        const std::string& delimiter) {
    std::vector<std::string> strings;
    std::string::size_type pos = 0;
    std::string::size_type prev = 0;
    while ((pos = str.find(delimiter, prev)) != std::string::npos) {
        strings.push_back(str.substr(prev, pos - prev));
        prev = pos + delimiter.size();
    }
    strings.push_back(str.substr(prev));
    if (strings[strings.size() - 1] == "") strings.pop_back();
    return strings;
}
