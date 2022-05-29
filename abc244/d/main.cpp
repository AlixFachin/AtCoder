#include <iostream>
using namespace std;

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

void swap(char *a, char *b)
{
    char c;
    c = (*a);
    *a = *b;
    *b = c;
}

int solveProblem()
{
    char s1, s2, s3, t1, t2, t3;

    cin >> s1 >> s2 >> s3;
    cin >> t1 >> t2 >> t3;

    int nbSwaps = 0;

    if (s1 != t1)
    {
        if (s1 == t2)
        {
            swap(&t1, &t2);
            nbSwaps++;
        }
        else
        {
            swap(&t1, &t3);
            nbSwaps++;
        }
    }
    // Now we have s1 == t1
    if (s2 != t2)
    {
        swap(&t2, &t3);
        nbSwaps++;
    }

    if (nbSwaps % 2 == 0)
    {
        cout << "Yes" << endl;
    }
    else
    {
        cout << "No";
    }

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
