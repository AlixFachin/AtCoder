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
    int a11, a12, a21, a22;
    int r, c;
    int ans;

    cin >> r >> c;
    cin >> a11 >> a12;
    cin >> a21 >> a22;
    ans = 0;
    if (r == 1 && c == 1)
    {
        ans = a11;
    }
    else if (r == 1 && c == 2)
    {
        ans = a12;
    }
    else if (r == 2 && c == 1)
    {
        ans = a21;
    }
    else if (r == 2 && c == 2)
    {
        ans = a22;
    }

    cout << ans << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
