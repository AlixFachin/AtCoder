#include <iostream>
#include <cmath>
using namespace std;

typedef long double ld;
typedef long long ll;

ll n = 0;

void dlog(string s)
{
#ifdef TEST
    cerr << s << endl;
#endif
}

ll f(ll a, ll b)
{
    return a * a * a + a * a * b + a * b * b + b * b * b - n;
}

struct point
{
    ll x;
    ll y;
    ll value;
};

void dPoint(string pointName, point pointArg)
{
#ifdef TEST
    dlog(" " + pointName + ": (" + to_string(pointArg.x) + "," + to_string(pointArg.y) + "):" + to_string(pointArg.value));
#endif
}

point setPoint(ll x, ll y)
{
    point res;

    res.x = x;
    res.y = y;
    res.value = f(x, y);

    return res;
}

ll getBruteForceMinimum(point a, point b, point c, point d)
{
    ll currentMin = INT32_MAX;
    ll currentValue;
    // For small squares, we just get the thing going and shut up
    for (int i = c.x; i <= b.x; i++)
    {
        for (int j = c.y; j <= b.y; j++)
        {
            currentValue = f(i, j);
            if (currentValue >= 0 && currentValue < currentMin)
            {
                currentMin = currentValue;
            }
        }
    }

    return currentMin;
}

// Semi brute force algorithm
ll algo2()
{
    ll currentMinimum = INT32_MAX;
    int j = 1E6;
    for (int i = 0; i < 1E6; i++)
    {
        if (f(i, 0) < currentMinimum)
        {
            while (j >= 0 && f(i, j) >= 0)
            {
                if (f(i, j) <= currentMinimum)
                {
                    currentMinimum = f(i, j);
                }
                j = j - 1;
            }
            j = j + 1;
        }
    }

    return currentMinimum;
}

/*
A --- B
|  M  |
C --- D
*/
ll getMinimum(point a, point b, point c, point d, ll currentMinimum)
{
    dlog("===========");
    dPoint("a", a);
    dPoint("b", b);
    dPoint("c", c);
    dPoint("d", d);

    // Break recursion Case 1 -> if one of the points is a minimum (cannot get better than zero!)
    if (a.value == 0 || b.value == 0 || c.value == 0 || d.value == 0)
    {
        cerr << "One point is equal to zero -> exiting!" << endl;
        return 0;
    }

    if (a.value > 0 && b.value > 0 && c.value > 0 && d.value > 0)
    {
        return c.value;
    }

    if (a.value < 0 && b.value < 0 && c.value < 0 && d.value < 0)
    {
        return b.value;
    }

    if (b.y - c.y <= 10 && b.x - c.x <= 10)
    {
        ll result = getBruteForceMinimum(a, b, c, d);
        cerr << "Small square! " << result << endl;
        // No need to get fancy for small areas
        return result;
    }

    if (c.value > currentMinimum)
    {
        // Every value in this area will be bigger than what we found already - aborting...
        return currentMinimum;
    }

    point m = setPoint((a.x + b.x) / 2, (a.y + c.y) / 2);
    dPoint("m", m);

    ll minA, minB;

    if (m.value > 0)
    {
        // looking in the lower quadrant first
        dlog("Looking at bottom-left quadrant");
        minB = getMinimum(setPoint(c.x, m.y), m, c, setPoint(m.x, c.y), currentMinimum);
        dlog("Receveived second minimum! (bottom-left)" + to_string(minB));
    }
    else
    { // looking in the top-right quadrant
        dlog("Looking at top-right quadrant");
        minB = getMinimum(setPoint(m.x, a.y), b, m, setPoint(d.x, m.y), currentMinimum);
        dlog("Receveived second minimum! (top-right)" + to_string(minB));
    }

    if (minB < currentMinimum && minB >= 0)
    {
        currentMinimum = minB;
    }

    // getting the minimum of the top-left quadrant
    dlog("Looking at top-left quadrant");
    minA = getMinimum(a, setPoint(m.x, a.y), setPoint(c.x, m.y), m, currentMinimum);
    dlog("Receveived first minimum! " + to_string(minA));

    if (minA < 0)
        return max(minA, minB);
    if (minB < 0)
        return minA;

    return min(minA, minB);
}

int solveProblem()
{
    cin >> n;

    /*point a, b, c, d;

    ll upLimit = min(n, 1000000LL);

    a = setPoint(0, upLimit);
    b = setPoint(upLimit, upLimit);
    c = setPoint(0, 0);
    d = setPoint(upLimit, 0);

    ll fMin = getMinimum(a, b, c, d, INT64_MAX);
    */
    ll fMin = algo2();

    cout << (fMin + n) << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
