#include <iostream>
using namespace std;

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

char invert(char src)
{
    if (src == '.')
    {
        return '#';
    }
    return '.';
}

int solveProblem()
{
    int n, a, b;
    char firstColChar;
    char currentChar;

    cin >> n >> a >> b;

    firstColChar = '#';
    currentChar = '#';

    for (int i = 0; i < (a * n); i++)
    {
        if (i % a == 0)
        {
            firstColChar = invert(firstColChar);
        }
        currentChar = invert(firstColChar);
        for (int j = 0; j < (b * n); j++)
        {
            if (j % b == 0)
            {
                currentChar = invert(currentChar);
            }
            cout << currentChar;
        }
        cout << endl;
    }

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
