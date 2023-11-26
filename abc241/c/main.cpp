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

int n;
vector<vector<int>> map;

bool checkVert(int y0)
{
    int totalSharp = 0;
    for (int k = 0; k < 6; k++)
    {
        totalSharp += map[k][y0];
    }
    if (totalSharp >= 4)
    {
        return true;
    }
    for (int i = 0; i < n - 6; i++)
    {
        totalSharp += map[i + 6][y0];
        totalSharp -= map[i][y0];
        if (totalSharp >= 4)
        {
            return true;
        }
    }
    return false;
}

bool checkHoriz(int i0)
{
    int totalSharp = 0;
    for (int j = 0; j < 6; j++)
    {
        totalSharp += map[i0][j];
    }
    if (totalSharp >= 4)
    {
        return true;
    }
    for (int j = 0; j < n - 6; j++)
    {
        // going to next
        totalSharp += map[i0][j + 6];
        totalSharp -= map[i0][j];
        if (totalSharp >= 4)
        {
            return true;
        }
    }
    return false;
}

bool checkDiagRight(int i0, int j0)
{
    int totalSharp = 0;
    for (int k = 0; k < 6; k++)
    {
        totalSharp += map[i0 + k][j0 + k];
    }

    if (totalSharp >= 4)
    {
        return true;
    }

    int i = i0;
    int j = j0;

    while (i < n - 6 && j < n - 6)
    {
        totalSharp += map[i + 6][j + 6];
        totalSharp -= map[i][j];
        if (totalSharp >= 4)
        {
            return true;
        }
        i++;
        j++;
    }
    return false;
}

bool CheckDiagLeft(int i0, int j0)
{
    int totalSharp = 0;
    for (int k = 0; k < 6; k++)
    {
        totalSharp += map[i0 + k][j0 - k];
    }

    if (totalSharp >= 4)
    {
        return true;
    }

    int i = i0;
    int j = j0;

    while (i < n - 6 && j >= 0)
    {
        totalSharp += map[i + 6][j - 6];
        totalSharp -= map[i][j];
        if (totalSharp >= 4)
        {
            return true;
        }
        i++;
        j--;
    }
    return false;
}

int solveProblem()
{
    cin >> n;

    // Global Variables init
    for (int i = 0; i < n; i++)
    {
        string s;
        cin >> s;

        vector<int> row;
        for (int j = 0; j < n; j++)
        {
            if (s[j] == '#')
            {
                row.push_back(1);
            }
            else
            {
                row.push_back(0);
            }
        }
        map.push_back(row);
    }

    // Checking horizontal and vertical
    for (int i = 0; i < n; i++)
    {
        if (checkHoriz(i) || checkVert(i))
        {
            cout << "Yes" << endl;
            return 0;
        }
    }

    // Third test -> diagonal tests!
    for (int j = 0; j < n - 6; j++)
    {
        if (checkDiagRight(j, 0) || checkDiagRight(0, j))
        {
            cout << "Yes" << endl;
            return 0;
        }
    }

    // Fourth test -> diagonal tests!
    for (int k = 0; k < n - 6; k++)
    {
        if (CheckDiagLeft(0, n - 1 - k) || CheckDiagLeft(k, n - 1))
        {
            cout << "Yes" << endl;
            return 0;
        }
    }

    cout << "No" << endl;
    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
