#include <iostream>
#include <filesystem>
#include <vector>
#include <fstream>
#include <sstream>
#include <algorithm>

#define TEST
#include "main.h"

using namespace std;
using std::filesystem::directory_iterator;

struct test_case
{
    string in;
    string out;
    string label;
};

string red(string message)
{
    return "\033[1;31m" + message + "\033[0m";
}

string green(string message)
{
    return "\033[32m" + message + "\033[0m";
}

string yellow(string message)
{
    return "\033[33m" + message + "\033[0m";
}

bool compare_tests(test_case t1, test_case t2)
{
    return (t1.label < t2.label);
}

string readFileIntoString(const string &path)
{
    ifstream input_file(path);
    if (!input_file.is_open())
    {
        cerr << "Could not open the file - '"
             << path << "'" << endl;
        exit(EXIT_FAILURE);
    }
    return string((std::istreambuf_iterator<char>(input_file)), std::istreambuf_iterator<char>());
}

void printVectorContent(std::vector<test_case> test_vector)
{
    cout << "------- Printing Vector -------\n";
    for (test_case &current_test : test_vector)
    {
        cout << "Test label: " << current_test.label << '\n';
        cout << "In:" << current_test.in << "\n";
        cout << "Out:" << current_test.out << "\n";
    }
    cout << "===================\n";
}

/*
 * addTestDescription:
 *  - Parse the filePath given in argument to get the filename and the test number
 *  - Looks into the vector to see if there is something defined
 */
void addTestDescription(std::vector<test_case> &test_vector, const std::filesystem::directory_entry &filePath)
{
    // 1) Parsing of the file name to get the test number
    string fileName = filePath.path().filename().string();
    size_t test_label_index = fileName.find_first_of('-') + 1;
    size_t dot_index = fileName.find_first_of('.');
    string testId = fileName.substr(test_label_index, dot_index - test_label_index);
    string fileType = fileName.substr(dot_index + 1);
    // cout << "Filename is: " << fileName << "," << testId << "," << fileType << "\n";

    // 2) Browsing the vector to see if we already have this test
    for (test_case &current_test : test_vector)
    {
        if (current_test.label == testId)
        {
            // cout << "Found the test, adding " << fileType << "\n";
            if (fileType == "out")
            {
                current_test.out = readFileIntoString(filePath.path().string());
            }
            else
            {
                current_test.in = readFileIntoString(filePath.path().string());
            }
            return;
        }
    }
    // if we are here it means we didn't find the test
    // cout << "Not found! Pushing at the end the test " << testId << "\n";
    test_case current_test;
    current_test.label = testId;
    if (fileType == "out")
    {
        current_test.out = readFileIntoString(filePath.path().string());
    }
    else
    {
        current_test.in = readFileIntoString(filePath.path().string());
    }

    test_vector.push_back(current_test);
}

int main()
{
    std::vector<test_case> test_vector;
    string path = "./tests/";

    for (const auto &file : directory_iterator(path))
    {
        addTestDescription(test_vector, file);
    }
    sort(test_vector.begin(), test_vector.end(), compare_tests);
    // printVectorContent(test_vector);

    bool allPassed = true;
    int testNr = 1;

    // Now, for each test, we will need to run the algo and compare the results
    for (const test_case &current_test : test_vector)
    {

        cout << yellow("Test number " + to_string(testNr) + " -=-=-=-=-=-=-= \n");
        cout << yellow(current_test.in) << endl;

        stringbuf in = stringbuf(current_test.in);
        streambuf *cinbuf = cin.rdbuf();
        cin.rdbuf(&in);

        stringbuf out;
        streambuf *coutbuf = cout.rdbuf();
        cout.rdbuf(&out);

        solveProblem();
        cin.rdbuf(cinbuf);
        cout.rdbuf(coutbuf);

        if (current_test.out != out.str())
        {
            cout << red("ERROR IN TEST!") << "\n[EXP]: \n"
                 << current_test.out << "\n[Actual]: \n"
                 << out.str() << "\n";
            allPassed = false;
        }
        else
        {
            cout << green("Test successful!") + "\n[EXP]: \n"
                 << current_test.out << "\n[Actual]: \n"
                 << out.str() << "\n";
        }

        testNr++;
    }

    cout << "=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\n";
    if (allPassed)
    {
        cout << green("ALL Tests Passed!!!\n");
    }
}
