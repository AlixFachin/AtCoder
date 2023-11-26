#include <iostream>
using namespace std;

#include <vector>
#include <unordered_set>

typedef long long ll;

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

// INT RANGE -> max int = 2,147,483,647 (so roughly 2E10)
// LONG LONG RANGE -> max value of INT64_MAX which is 9,223,372,036,854,775,807 (roughly 9E16)

// -=-=- Logging functions -=-=-=-=-=-=
template <typename T>
void logVector(string vectorName, vector<T> *v)
{
#ifdef TEST
    cerr << "The content of " << vectorName << " is:" << endl;
    cerr << "[";
    for (int i = 0; i < v->size() - 1; i++)
    {
        cerr << (*v)[i] << ",";
    }
    cerr << (*v)[(v->size() - 1)] << "]" << endl;
#endif
}

template <typename T>
void logSet(string setName, unordered_set<T> *s)
{
#ifdef TEST
    cerr << "The content of " << setName << " is:" << endl;
    cerr << "{";
    for (const auto &elem : *s)
    {
        cerr << elem << ",";
    }

    cerr << "}" << endl;
#endif
}

void log(string s)
{
#ifdef TEST
    cerr << s << endl;
#endif
}
// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

int solveProblem()
{
    int n;
    vector<vector<char>> a;

    cin >> n;

    for (int i = 0; i < n; i++)
    {
        // Reading the data
        vector<char> curRow;
        string s;
        cin >> s;
        for (int j = 0; j < n; j++)
        {
            curRow.push_back(s[j]);
        }
        a.push_back(curRow);
    }

    string maxNumber = "0";
    // Looking at all the possilities
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            string curNumber = "";
            // Vertical (lower)
            for (int k = 0; k < n; k++)
            {
                int row = (i + k) % n;
                int col = j;
                char digit = a[row][col];
                curNumber += digit;
            }
            log("Vert low" + to_string(i) + "," + to_string(j) + ":" + curNumber);
            if (curNumber > maxNumber)
            {
                maxNumber = curNumber;
            }
            // Vertical (higher)
            curNumber = "";
            for (int k = 0; k < n; k++)
            {
                int row = (i - k);
                if (row < 0)
                {
                    row += n;
                }
                int col = j;
                int digit = a[row][col];
                curNumber += digit;
            }
            if (curNumber > maxNumber)
            {
                maxNumber = curNumber;
            }

            // Horizontal (left)
            curNumber = "";
            for (int k = 0; k < n; k++)
            {
                int row = i;
                int col = (j - k);
                if (col < 0)
                    col += n;
                int digit = a[row][col];
                curNumber += digit;
            }
            if (curNumber > maxNumber)
            {
                maxNumber = curNumber;
            }

            // Horizontal (right)
            curNumber = "";
            for (int k = 0; k < n; k++)
            {
                int row = i;
                int col = (j + k) % n;
                int digit = a[row][col];
                curNumber += digit;
            }
            if (curNumber > maxNumber)
            {
                maxNumber = curNumber;
            }
            // Diagonal 1 (NW)
            curNumber = "";
            for (int k = 0; k < n; k++)
            {
                int row = (i - k);
                int col = (j - k);
                if (row < 0)
                    row += n;
                if (col < 0)
                    col += n;
                int digit = a[row][col];
                curNumber += digit;
            }
            if (curNumber > maxNumber)
            {
                maxNumber = curNumber;
            }

            // Diagonal 2 (NE)
            curNumber = "";
            for (int k = 0; k < n; k++)
            {
                int row = (i - k);
                if (row < 0)
                    row += n;
                int col = (j + k) % n;
                int digit = a[row][col];
                curNumber += digit;
            }
            if (curNumber > maxNumber)
            {
                maxNumber = curNumber;
            }

            // Diagonal 3 (SW)
            curNumber = "";
            for (int k = 0; k < n; k++)
            {
                int row = (i + k) % n;
                int col = (j - k);
                if (col < 0)
                    col += n;
                int digit = a[row][col];
                curNumber += digit;
            }
            if (curNumber > maxNumber)
            {
                maxNumber = curNumber;
            }

            // Diagonal 4 (SE)
            curNumber = "";
            for (int k = 0; k < n; k++)
            {
                int row = (i + k) % n;
                int col = (j + k) % n;
                int digit = a[row][col];
                curNumber += digit;
            }
            if (curNumber > maxNumber)
            {
                maxNumber = curNumber;
            }
        }
    }

    cout << maxNumber << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
