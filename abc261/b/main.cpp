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

bool isOpposite(char a, char b)
{
    if (a == 'W')
    {
        return (b == 'L');
    };
    if (a == 'L')
    {
        return (b == 'W');
    };
    if (a == 'D')
    {
        return (b == 'D');
    }
    return false;
}

int solveProblem()
{
    int n;
    cin >> n;

    vector<string> games;

    for (int i = 0; i < n; i++)
    {
        string currentGames;
        cin >> currentGames;
        games.push_back(currentGames);
    }

    bool isCorrect = true;
    for (int i = 0; i < n; i++)
    {
        for (int j = i + 1; j < n; j++)
        {
            char firstResult = games[i][j];
            char secondResult = games[j][i];
            if (!isOpposite(firstResult, secondResult))
            {
                isCorrect = false;
                break;
            }
        }
    }

    if (isCorrect)
    {
        cout << "correct" << endl;
    }
    else
    {
        cout << "incorrect" << endl;
    }

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
