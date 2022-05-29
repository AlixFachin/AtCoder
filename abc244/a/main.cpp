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
    int n;
    string s;
    cin >> n;
    cin >> s;

    cout << s.at(s.length() - 1) << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
