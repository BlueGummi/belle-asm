#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <filesystem>
#include <stdexcept>

namespace fs = std::filesystem;

std::string formatLine(const std::string& line) {
    if (line.find(';') != std::string::npos) return "";
    if (line.back() == ':' || line.front() == '.') return line;
    return "    " + line;
}

std::vector<std::string> readFileSafe(const std::string& filePath) {
    std::ifstream file(filePath);
    if (!file) throw std::runtime_error("Error reading file");
    
    std::vector<std::string> lines;
    std::string line;
    while (std::getline(file, line)) {
        lines.push_back(line);
    }
    return lines;
}

void writeFile(const std::string& filePath, const std::vector<std::string>& lines) {
    std::ofstream file(filePath);
    for (const auto& line : lines) {
        file << line << "\n";
    }
}

void processInputFile(const std::string& inputFilePath) {
    if (!fs::exists(inputFilePath)) return;

    try {
        auto fileContents = readFileSafe(inputFilePath);
        std::vector<std::string> formattedLines;
        for (const auto& line : fileContents) {
            formattedLines.push_back(formatLine(line));
        }

        std::string tempFilePath = inputFilePath + ".tmp";
        writeFile(tempFilePath, formattedLines);

        fs::rename(tempFilePath, inputFilePath);
    } catch (const std::exception& e) {
        std::cerr << e.what() << std::endl;
        if (fs::exists(inputFilePath + ".tmp")) {
            fs::remove(inputFilePath + ".tmp");
        }
    }
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cout << "Usage: " << argv[0] << " <input>" << std::endl;
        return 1;
    }

    for (int i = 1; i < argc; ++i) {
        processInputFile(argv[i]);
    }

    return 0;
}
