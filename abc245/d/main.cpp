#include <iostream>
using namespace std;
typedef long long ll;

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

int a[200];
int b[200];
int c[200];

int n, m;

long getCrossSumCoeff(int k)
{

    ll result = 0;
    for (int j = 1; j <= k; j++)
    {
        if (n - k + j >= 0)
        {
            result += a[n - k - 1 + j] * b[m - j + 1];
        }
    }
    return result;
}

int solveProblem()
{

    cin >> n >> m;
    for (int i = 0; i <= n; i++)
    {
        cin >> a[i];
    }
    for (int i = 0; i <= n + m; i++)
    {
        cin >> c[i];
    }

    b[m] = c[n + m] / a[n];

    for (int j = 1; j <= m; j++)
    {
        b[m - j] = (c[n + m - j] - getCrossSumCoeff(j)) / a[n];
    }

    // Outputing final coefficients
    for (int j = 0; j < m; j++)
    {
        cout << b[j] << " ";
    }
    cout << b[m] << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
