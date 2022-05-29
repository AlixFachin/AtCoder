#include <iostream>
using namespace std;

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

int solveProblem()
{
    int a, b, c, d;
    const string name1 = "Takahashi";
    const string name2 = "Aoki";
    cin >> a >> b >> c >> d;

    if (a < c)
    {
        cout << name1 << endl;
        return 0;
    }
    else if (c < a)
    {
        cout << name2 << endl;
        return 0;
    }
    // same hour - now checking minutes
    if (b <= d)
    {
        cout << name1 << endl;
    }
    else if (d < b)
    {
        cout << name2 << endl;
    }

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
