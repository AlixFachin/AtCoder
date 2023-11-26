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

void logMsg(string s)
{
#ifdef TEST
    cerr << s << endl;
#endif
}
// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

int solveProblem()
{
    int n, q;
    cin >> n >> q;

    vector<ll> elements;
    vector<ll> distance;

    for (int i = 0; i < n; i++)
    {
        ll a;
        cin >> a;
        elements.push_back(a);
    }
    sort(elements.begin(), elements.end());

    ll dist = 0;
    for (int j = 1; j < n; j++)
    {
        dist += abs(elements[j] - elements[0]);
    }
    distance.push_back(dist);

    for (int i = 1; i < n; i++)
    {
        dist = distance[i - 1] + (i) * (elements[i] - elements[i - 1]) - (n - i) * (elements[i] - elements[i - 1]);
        distance.push_back(dist);
    }

    logVector("el", &elements);
    logVector("d", &distance);

    for (int i = 0; i < q; i++)
    {
        ll x;
        cin >> x;
        ll ans = 0;

        if (x <= elements[0])
        {
            ans = distance[0] + abs(x - elements[0]) * n;
        }
        else if (x >= elements[n - 1])
        {
            ans = distance[n - 1] + abs(x - elements[n - 1]) * n;
        }
        else
        {
            // binary search!
            int l = lower_bound(elements.begin(), elements.end(), x) - elements.begin() - 1;
            logMsg("Looking for " + to_string(x) + ", found index " + to_string(l) + " bounded by: " + to_string(elements[l]) + " and " + to_string(elements[l + 1]));
            if (x == elements[l])
            {
                ans = distance[x];
            }
            else if (x > elements[l] && x <= elements[l + 1])
            {
                ans = distance[l] + abs(x - elements[l]) * (l + 1) - abs(x - elements[l]) * (n - 1 - l);
            }
        }

        cout << ans << endl;
    }

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
