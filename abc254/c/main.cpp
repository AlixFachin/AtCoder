#include <iostream>
using namespace std;

#include <vector>
#include <unordered_set>
#include <algorithm>

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
    int n, k;
    cin >> n >> k;

    if (k == 1)
    {
        cout << "Yes" << endl;
        return 0;
    }

    vector<vector<int>> subArrays(k);

    for (int i = 0; i < n; i++)
    {
        int a;
        cin >> a;
        subArrays[(i % k)].push_back(a);
    }

    for (int i = 0; i < k; i++)
    {
        // logVector("subarray " + to_string(i), &subArrays[i]);
        sort(subArrays[i].begin(), subArrays[i].end());
    }

    /*for (int i = 0; i < k; i++)
    {
        logVector("subarray " + to_string(i), &subArrays[i]);
    } */

    for (int i = 0; i < n - 1; i++)
    {
        // Looking at the nth element
        int rowIndex = i % k;
        int colIndex = i / k;
        int rowIndex_next = (i + 1) % k;
        int colIndex_next = (i + 1) / k;
        // log("Testing " + to_string(i) + " at (" + to_string(rowIndex) + "," + to_string(colIndex) + ") = " + to_string(subArrays[rowIndex][colIndex]));
        // log("     and (" + to_string(rowIndex_next) + "," + to_string(colIndex_next) + ") = " + to_string(subArrays[rowIndex_next][colIndex_next]));
        if (subArrays[rowIndex][colIndex] > subArrays[rowIndex_next][colIndex_next])
        {
            // log("NOPE");
            cout << "No" << endl;
            return 0;
        }
    }

    cout << "Yes" << endl;
    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
