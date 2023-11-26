#include <iostream>
using namespace std;

#include <vector>
#include <unordered_set>
#include <algorithm>

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
    if (v->size() == 0)
    {
        cerr << "empty" << endl;
        return;
    }
    cerr << "[";
    for (int i = 0; i < v->size() - 1; i++)
    {
        cerr << (*v)[i] << ",";
    }
    cerr << (*v)[(v->size() - 1)] << "]" << endl;
#endif
}

template <typename T>
void logPairVector(string vectorName, vector<pair<T, T>> *v)
{
#ifdef TEST
    cerr << "The content of " << vectorName << " is:" << endl;
    cerr << "[";
    for (int i = 0; i < v->size() - 1; i++)
    {
        cerr << "(" << v->at(i).first << "," << v->at(i).second << ")"
             << ",";
    }
    cerr << "(" << v->at(v->size() - 1).first << "," << v->at(v->size() - 1).second << ")"
         << "]" << endl;
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
    cin >> n;

    vector<pair<double, int>> intArray;
    vector<pair<double, double>> finalArray;

    for (int i = 0; i < n; i++)
    {
        double l, r;
        cin >> l >> r;
        intArray.push_back({l, 1});
        intArray.push_back({r, -1});
    }

    sort(intArray.begin(), intArray.end());

    int nrIntervals = 0;
    double intervalBegin = -1;
    for (int i = 0; i < intArray.size(); i++)
    {
        auto [v, d] = intArray[i];
        int nextValue = nrIntervals + d;
        if (nrIntervals == 0 && nextValue != 0)
        {
            intervalBegin = v;
        }
        else if (nrIntervals > 0 && nextValue == 0)
        {
            finalArray.push_back({intervalBegin, v});
        }
        nrIntervals = nextValue;
    }

    for (int i = 0; i < finalArray.size(); i++)
    {
        auto [x, y] = finalArray[i];
        cout << x << " " << y << endl;
    }

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
