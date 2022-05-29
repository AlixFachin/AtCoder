#include <iostream>
#include <unordered_map>

using namespace std;

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

int solveProblem()
{
    int n; // n <2000
    unordered_map<int, bool> ref_table;

    cin >> n;

    for (int i = 0; i < n; i++)
    {
        int a;
        cin >> a;
        ref_table[a] = true;
    }

    // Now find the smallest integer not in the list
    int i = 0;
    while (i <= 2001 && ref_table.count(i))
    {
        i++;
    }

    cout << i << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
