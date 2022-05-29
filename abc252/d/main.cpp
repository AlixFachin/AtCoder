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
    vector<int> a;

    cin >> n;
    for (int i = 0; i < n; i++)
    {
        int elem;
        cin >> elem;
        a.push_back(elem);
    }

    // The problem is similar to looking at the number of possibilities where Ai < Aj < Ak

    const int MAX_VAL = 2 * 1E5;
    ll count_smaller_than[MAX_VAL + 1] = {0};

    for (int i = 0; i < n; i++)
    {
        count_smaller_than[a[i]] += 1;
    }

#ifdef TEST

    for (int i = 0; i < min(MAX_VAL, 100); i++)
    {
        cerr << to_string(count_smaller_than[i]) << ",";
    }
    cerr << endl;

#endif

    // Computing a running sum of all the values to effectively get the smallest
    for (int i = 0; i < MAX_VAL; i++)
    {
        count_smaller_than[i + 1] += count_smaller_than[i];
    }

    ll result = 0;
    for (int i = 0; i < n; i++)
    {
        // cerr << "Step " + to_string(i) + ", value= " + to_string(a[i]) << endl;
        // cerr << "Smaller: " + to_string(count_smaller_than[a[i] - 1]) + ", bigger: " + to_string(n - count_smaller_than[a[i]]) << endl;
        result += (count_smaller_than[a[i] - 1]) * (n - count_smaller_than[a[i]]);
    }

    cout << result << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
