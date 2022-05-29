#include <iostream>
using namespace std;
typedef long long ll;
#include <vector>

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

int solveProblem()
{
    ll n, k;
    cin >> n >> k;

    vector<ll> a;
    vector<ll> b;

    for (int i = 0; i < n; i++)
    {
        ll element;
        cin >> element;
        a.push_back(element);
    }
    for (int i = 0; i < n; i++)
    {
        ll element;
        cin >> element;
        b.push_back(element);
    }

    // We will browse both lists.
    // The only reason it would fail would be if whatever choice we make for Xi, the points afterwards are too far away
    for (int i = 0; i < n - 1; i++)
    {
        ll a0 = a[i];
        ll b0 = b[i];
        ll a1 = a[i + 1];
        ll b1 = b[i + 1];

        // Should a1 be disqualified?
        if (abs(a1 - b0) > k && abs(a1 - a0) > k)
        {
            // then a1 is far away from both next points -> need to flag it and replace it with b1.
            a[i + 1] = b1;
            if ((abs(b1 - b0) > k) && (abs(b1 - a0) > k))
            {
                // both a1 and b1 are too far - we can exit now
                cout << "No" << endl;
                return -1;
            }
        }
        else if (abs(b1 - b0) > k && abs(b1 - a0) > k)
        {
            // B1 point is too far - we will replace it with the A1 to check for the following iteration
            b[i + 1] = a1;
        }
    }

    cout << "Yes" << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
