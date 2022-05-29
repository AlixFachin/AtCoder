#include <iostream>
using namespace std;

#include <vector>
#include <unordered_map>

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

int solveProblem()
{
    int n, q;
    cin >> n >> q;
    vector<int> operations;

    for (int i = 0; i < q; i++)
    {
        int operation;
        cin >> operation;
        operations.push_back(operation);
    }

    vector<int> ballList;
    unordered_map<int, int> locations;
    for (int i = 0; i < n; i++)
    {
        ballList.push_back(i + 1);
        locations[i + 1] = i;
    }

    for (int i = 0; i < q; i++)
    {
        int x = operations[i];
        int x_index = locations[x];
        int y, y_index;
        if (x_index == n - 1)
        {
            y_index = max(0, n - 2);
        }
        else
        {
            y_index = x_index + 1;
        }
        y = ballList[y_index];

        // Now we swap x and y
        ballList[x_index] = y;
        ballList[y_index] = x;
        locations[x] = y_index;
        locations[y] = x_index;
    }

    for (int i = 0; i < n - 1; i++)
    {
        cout << ballList[i] << ' ';
    }
    cout << ballList[n - 1] << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
