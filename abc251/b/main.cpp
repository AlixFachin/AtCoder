#include <iostream>
#include <unordered_set>
#include <vector>
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
    unordered_set<ll> element_sum;
    vector<int> weights;
    int n, w;

    cin >> n >> w;
    for (int i = 0; i < n; i++)
    {
        ll element;
        cin >> element;
        weights.push_back(element);
    }

    // Now, browsing loop for all elements
    ll element;
    for (int i = 0; i < n; i++)
    {
        element = weights[i];
        if (element <= w)
        {
            element_sum.insert(weights[i]);
            for (int j = i + 1; j < n; j++)
            {
                element = weights[i] + weights[j];
                if (element <= w)
                {
                    element_sum.insert(weights[i] + weights[j]);
                    for (int k = j + 1; k < n; k++)
                    {
                        element = weights[i] + weights[j] + weights[k];
                        if (element <= w)
                        {
                            element_sum.insert(weights[i] + weights[j] + weights[k]);
                        }
                    }
                }
            }
        }
    }

    cout << element_sum.size() << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
