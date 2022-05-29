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
    int r, h, c, w;
    int x, y;

    cin >> h >> w;
    cin >> r >> c;

    if (h == 1)
    {
        y = 0;
    }
    else if (r == 1 || r == h)
    {
        y = 1;
    }
    else
    {
        y = 2;
    }

    if (w == 1)
    {
        x = 0;
    }
    else if (c == 1 || c == w)
    {
        x = 1;
    }
    else
    {
        x = 2;
    }

    cout << (x + y) << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
