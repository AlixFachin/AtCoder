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
    string t;

    int x = 0;
    int y = 0;
    int dir_x = +1;
    int dir_y = 0;

    cin >> n;
    cin >> t;

    for (int i = 0; i < t.length(); i++)
    {
        char act = t.at(i);
        if (act == 'S')
        {
            x += dir_x;
            y += dir_y;
        }
        else
        {
            if (dir_x == 1 && dir_y == 0)
            {
                dir_x = 0;
                dir_y = -1;
            }
            else if (dir_x == 0 && dir_y == -1)
            {
                dir_x = -1;
                dir_y = 0;
            }
            else if (dir_x == -1 && dir_y == 0)
            {
                dir_x = 0;
                dir_y = 1;
            }
            else if (dir_x == 0 && dir_y == 1)
            {
                dir_x = +1;
                dir_y = 0;
            }
            else
            {
                cerr << "Issue with the character " + to_string(i) + act << endl;
            }
        }
    }
    cout << x << " " << y << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
