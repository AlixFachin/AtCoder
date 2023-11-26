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
    ll x, a, d, n;
    ll ans;

    cin >> x >> a >> d >> n;

    // Cases where X is outside the [a. a+dn] interval
    ll maxTerm = max(a, a + (n - 1) * d);
    ll minTerm = min(a, a + (n - 1) * d);

    if (x >= maxTerm)
    {
        ans = x - maxTerm;
    }
    else if (x <= minTerm)
    {
        ans = minTerm - x;
    }
    else if (d == 0)
    {
        ans = abs(x - a);
    }
    else
    {
        ll myMod = (x - a) % d;
        ll myDiv = (x - a) / d;

        ll term1, term2, term3;
        term1 = (x - a) - (myDiv - (abs(myDiv) > 0 ? 1 : 0)) * d;
        term2 = (x - a) - (myDiv * d);
        term3 = (x - a) - (myDiv + (abs(myDiv) < n ? 1 : 0)) * d;

        ans = min(abs(term1), min(abs(term2), abs(term3)));
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
