#include <iostream>
using namespace std;

#include <vector>
#include <unordered_set>
#include <unordered_map>
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

template <typename T>
void logMap(string mapName, unordered_map<int, T> *m)
{
#ifdef TEST
    cerr << "Content of " + mapName << endl;
    for (const auto &key_value : *m)
    {
        cerr << "  " + to_string(key_value.first) + ": " + to_string(key_value.second) << endl;
    }
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

    // Setting up the parameter
    cin >> n;

    vector<vector<int>> occurences(n + 1);
    for (int i = 0; i < n; i++)
    {
        int a;
        cin >> a;

        occurences[a].push_back(i);
    }
    // logVector("numbers", &numbers);
    // logMap("first memo", &memo[0]);

    // Looking for the answer
    int q;
    cin >> q;

    for (int i = 0; i < q; i++)
    {
        int l, r, x;
        cin >> l >> r >> x;

        // log("Lower bound: " + to_string(lower_bound(freq.begin(), freq.end(), r) - freq.begin()));
        // log("Upper bound: " + to_string(lower_bound(freq.begin(), freq.end(), l - 1) - freq.begin()));

        cout << lower_bound(occurences[x].begin(), occurences[x].end(), r) - lower_bound(occurences[x].begin(), occurences[x].end(), l - 1) << endl;
    }

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
