#include <iostream>
using namespace std;

#include <vector>
#include <unordered_set>

#include <regex>

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
    int n;
    ll x;
    string s;

    cin >> n >> x;
    cin >> s;

    vector<char> operationStack;
    for (int i = 0; i < s.size(); i++)
    {
        if (s[i] == 'L' || s[i] == 'R')
        {
            operationStack.push_back(s[i]);
        }
        else
        {
            if (operationStack.size() > 1 &&
                (operationStack.back() == 'R' || operationStack.back() == 'L'))
            {
                operationStack.pop_back();
            }
            else
            {
                operationStack.push_back('U');
            }
        }
    }

    for (int i = 0; i < operationStack.size(); i++)
    {
        char d;
        d = operationStack[i];
        if (d == 'U')
        {
            x = x / 2;
        }
        else if (d == 'L')
        {
            x = 2 * x;
        }
        else if (d == 'R')
        {
            x = 2 * x + 1;
        }
    }

    cout << x << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
