#include <iostream>
using namespace std;
#include <vector>
#include <algorithm>
#include <iomanip>

typedef long long ll;

int solveProblem()
{
    ll n, k, x;

    cin >> n >> k >> x;

    // Reading the items in order
    std::vector<ll> item_prices;
    for (int i = 0; i < n; i++)
    {
        ll item;
        cin >> item;
        item_prices.push_back(item);
    }

    // First pass -> how many full coupons can we get?
    ll numFullCoupons = 0;
    ll fullPriceSum = 0;
    for (int i = 0; i < n; i++)
    {
        numFullCoupons += (item_prices.at(i) / x);
        fullPriceSum += item_prices.at(i);
    }

    if (k <= numFullCoupons)
    {
        // not enough coupons for the second pass
        ll result = fullPriceSum - (k * x);
        cout << result << endl;
        return 0;
    }

    ll partialCouponsCount = k - numFullCoupons;
    if (partialCouponsCount >= n)
    {
        // more coupons than remaining elements -> we can get all the items for free!
        cout << 0 << endl;
        return 0;
    }

    // We need to do a second pass, with the modulos
    std::vector<ll> modulo_array;
    for (int i = 0; i < n; i++)
    {
        modulo_array.push_back((item_prices.at(i) % x));
    }

    // We will take the sum of all the items on which we won't apply coupons, i.e. the
    // smallest ones
    sort(modulo_array.begin(), modulo_array.end());
    ll moduloSum = 0;
    for (int i = 0; i < n - partialCouponsCount; i++)
    {
        moduloSum += modulo_array.at(i);
    }

    cout << moduloSum << endl;
    // cout << ans << endl;
    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
