#include <iostream>
using namespace std;
#include <unordered_set>
#include <vector>
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

ll bigComp(ll x)
{
    if (x % 2 == 0)
    {
        return (x / 2) * (x + 1);
    }
    else
    {
        return x * ((x + 1) / 2);
    }
}

ll gcd(ll a, ll b)
{
    if (a == 0)
    {
        return b;
    }
    return gcd(b % a, a);
}

ll lcm(ll a, ll b)
{
    return (a * b) / (gcd(a, b));
}

int solveProblem()
{
    ll n, a, b;

    cin >> n >> a >> b;

    ll result = 0;

    ll normalSum = bigComp(n);
    ll aSum = 0;
    ll bSum = 0;
    ll abSum = 0;
    if (a <= n)
    {
        ll aDiv = (n / a);
        aSum = a * bigComp(aDiv);
    }
    if (b <= n)
    {
        ll bDiv = (n / b);
        bSum = b * bigComp(bDiv);
    }
    ll abDiv = (n / lcm(a, b));
    abSum = lcm(a, b) * bigComp(abDiv);

    result = normalSum - aSum - bSum + abSum;

    std::cout << result << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
