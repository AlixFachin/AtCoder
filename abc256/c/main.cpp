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

int h1, h2, h3, w1, w2, w3;

bool isGridOK(int a11, int a21, int a12, int a32)
{
    int a13, a22, a23, a31, a33;

    a13 = h1 - a12 - a11;
    a22 = w2 - a12 - a32;
    a23 = h2 - a21 - a22;
    a31 = w1 - a11 - a21;
    a33 = w3 - a13 - a23;

    if (a33 != (h3 - a31 - a32))
    {
        // broken grid
        return false;
    }

    if (a13 > 0 && a22 > 0 && a23 > 0 && a31 > 0 && a33 > 0)
    {
        return true;
    }
    return false;
}

int solveProblem()
{
    cin >> h1;
    cin >> h2;
    cin >> h3;
    cin >> w1;
    cin >> w2;
    cin >> w3;

    ll ans = 0;

    for (int a11 = 1; a11 < 29; a11++)
    {
        for (int a21 = 1; a21 < 29; a21++)
        {
            for (int a12 = 1; a12 < 29; a12++)
            {
                for (int a32 = 1; a32 < 29; a32++)
                {
                    if (isGridOK(a11, a21, a12, a32))
                    {
                        ans++;
                    }
                }
            }
        }
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
