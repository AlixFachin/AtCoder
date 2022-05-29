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
    string s;
    int n, k;

    cin >> s;

    cerr << "MAX INT16:" << INT16_MAX << endl;
    cerr << "MAX INT32:" << INT32_MAX << endl;
    cerr << "MAX INT64:" << INT64_MAX << endl;

    n = s.length();
    for (int i = 0; i < 6 / n; i++)
    {
        cout << s;
    }
    cout << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
