#include <iostream>
using namespace std;

#include <vector>
#include <unordered_set>
#include <queue>

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

    struct Element
    {
        ll value;
        ll count;
    };

    queue<Element> cylinder;

    bool displayed = false;

    int q;
    cin >> q;
    while (q--)
    {
        int op;
        ll c, x;
        Element elem;

        cin >> op;
        if (op == 1)
        {
            cin >> x >> c;
            elem.count = c;
            elem.value = x;
            cylinder.push(elem);
        }
        else
        {
            cin >> c;
            ll res = 0;
            while (c > 0)
            {
                elem = cylinder.front();
                if (elem.count >= c)
                {
                    res += c * elem.value;
                    cylinder.front().count -= c;
                    c = 0;
                }
                else
                {
                    res += elem.count * elem.value;
                    cylinder.pop();
                    c -= elem.count;
                }
            }
            cout << res << endl;
            displayed = true;
        }
    }

    if (!displayed)
    {
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
