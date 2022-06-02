#include <iostream>
using namespace std;

#include <vector>
#include <unordered_map>
#include <cmath>
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

template <typename K, typename V>
void logMap(string mapName, unordered_map<K, V> *s)
{
#ifdef TEST
    cerr << "The content of " << mapName << " is:" << endl;
    cerr << "{";
    for (const auto &key_value : *s)
    {
        cerr << to_string(key_value.first) << ":" << to_string(key_value.second) << ",";
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
// -=-=-=-=-=-= INPUT SOLUTION CODE BELOW -=-=-=-=-=-=-=-=-=-=-=-=

const ll ansMod = 998244353;
const int MAX_N = 50;

vector memo(MAX_N + 1, vector(MAX_N *MAX_N + 1, -1));

ll dp(int n, int m, int k)
{
    log("Checking " + to_string(n) + "," + to_string(m) + "," + to_string(k));
    if (k < n)
    {
        return 0;
    }

    if (n == 1)
    {
        return min(k, m);
    }

    if (memo[n][k] != -1)
    {
        log("Found memo - returning!");
        return memo[n][k];
    }

    ll res = 0;
    for (int j = 1; j <= m; j++)
    {
        log("Calling " + to_string(n - 1) + "," + to_string(m) + "," + to_string(k - j));
        res = (res + dp(n - 1, m, k - j)) % ansMod;
    }

    memo[n][k] = res;
    return res;
}

int solveProblem()
{
    int n, m, k;
    cin >> n >> m >> k;

    ll res = dp(n, m, k);

    cout << res << endl;

    return 0;
}

#ifndef TEST

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
int main(void)
{
    solveProblem();
}
#endif
