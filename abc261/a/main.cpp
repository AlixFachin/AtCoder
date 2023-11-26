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
    int l1, r1, l2, r2;

    cin >> l1 >> r1 >> l2 >> r2;

    int m1 = min(l1, l2);
    int m2 = max(r1, r2);
    int count = 0;

    log(to_string(l1) + "," + to_string(l2) + "," + to_string(r1) + "," + to_string(r2) + ":" + to_string(m1) + "," + to_string(m2));

    for (int i = m1; i <= m2; i++)
    {
        log("Testing " + to_string(i));
        if (i > l1 && i <= r1 && i > l2 && i <= r2)
        {
            log("  MAtches " + to_string(i) + "(" + to_string(count) + ")");
            count = count + 1;
        }
    }
    log(to_string(count));

    cout << count << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
