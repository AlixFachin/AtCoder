#include <iostream>
#include <unordered_map>
using namespace std;
typedef long long ll;
// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

int solveProblem()
{
    int n;
    unordered_map<string, ll> scoreTable;

    ll maxScore = 0;
    int maxIndex = 0;

    cin >> n;

    for (int i = 0; i < n; i++)
    {
        string s;
        ll t;
        cin >> s >> t;
        if (!scoreTable.count(s))
        {
            scoreTable[s] = t;
            if (t > maxScore)
            {
                maxScore = t;
                maxIndex = i;
            }
        }
    }

    cout << (maxIndex + 1) << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
