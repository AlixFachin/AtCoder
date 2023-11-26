#include <iostream>
using namespace std;

#include <vector>
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

void log(string s)
{
#ifdef TEST
    cerr << s << endl;
#endif
}
// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

int solveProblem()
{
    int n, m, t;
    cin >> n >> m >> t;

    vector<int> roomTime;
    unordered_map<int, int> bonuses;

    for (int i = 0; i < n - 1; i++)
    {
        int a;
        cin >> a;
        roomTime.push_back(a);
    }

    for (int i = 0; i < m; i++)
    {
        int x, y;
        cin >> x >> y;
        bonuses[x] = y;
    }

    // Now browsing the time
    ll timeRemaining = t;
    int position = 0;
    while ((timeRemaining > 0) && (position < n - 1))
    {
        if (bonuses.count((position + 1)))
        {
            timeRemaining += bonuses.at(position + 1);
        }
        timeRemaining -= roomTime[position];
        if (timeRemaining > 0)
            position++;
    }

    if (position == (n - 1))
    {
        cout << "Yes" << endl;
    }
    else
    {
        cout << "No" << endl;
    }

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
