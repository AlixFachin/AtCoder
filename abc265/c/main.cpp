#include <iostream>
using namespace std;

#include <vector>
#include <unordered_set>

typedef long long ll;

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

// INT RANGE -> max int = 2,147,483,647 (so roughly 2E10)
// LONG LONG RANGE -> max value of INT64_MAX which is 9,223,372,036,854,775,807 (roughly 9E16)

// -=-=- Logging functions -=-=-=-=-=-=
template <typename T>
void logVector(string vectorName, vector<T> *v)
{
#ifdef TEST
    cerr << "The content of " << vectorName << " is:" << endl;
    cerr << "[";
    for (int i = 0; i < v->size() - 1; i++)
    {
        cerr << (*v)[i] << ",";
    }
    cerr << (*v)[(v->size() - 1)] << "]" << endl;
#endif
}

template <typename T>
void logSet(string setName, unordered_set<T> *s)
{
#ifdef TEST
    cerr << "The content of " << setName << " is:" << endl;
    cerr << "{";
    for (const auto &elem : *s)
    {
        cerr << elem << ",";
    }

    cerr << "}" << endl;
#endif
}

void log(string s)
{
#ifdef TEST
    cerr << s << endl;
#endif
}
// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

int solveProblem()
{
    int h, w;
    vector<string> directions;
    vector<vector<bool>> visited;

    cin >> h >> w;
    for (int i = 0; i < h; i++)
    {
        string s;
        cin >> s;
        directions.push_back(s);
        visited.push_back({});
        for (int j = 0; j < w; j++)
        {
            visited[i].push_back(false);
        }
    }

    int i = 1, j = 1;
    int nexti, nextj;
    int canMove = true;
    while (canMove)
    {
        char nextMove = directions[i - 1][j - 1];
        if (nextMove == 'U')
        {
            if (i == 1)
            {
                canMove = false;
                continue;
            }
            nexti = i - 1;
            nextj = j;
        }
        else if (nextMove == 'D')
        {
            if (i == h)
            {
                canMove = false;
                continue;
            }
            nexti = i + 1;
            nextj = j;
        }
        else if (nextMove == 'R')
        {
            if (j == w)
            {
                canMove = false;
                continue;
            }
            nexti = i;
            nextj = j + 1;
        }
        else if (nextMove == 'L')
        {
            if (j == 1)
            {
                canMove = false;
                continue;
            }
            nexti = i;
            nextj = j - 1;
        }
        if (visited[nexti - 1][nextj - 1])
        {
            cout << "-1" << endl;
            return 0;
        }
        visited[i - 1][j - 1] = true;
        i = nexti;
        j = nextj;
    }
    cout << i << " " << j << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
