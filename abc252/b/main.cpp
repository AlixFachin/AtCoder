#include <iostream>
using namespace std;

#include <vector>
#include <unordered_set>

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
    int n, k;
    vector<int> a;
    vector<int> b;

    cin >> n >> k;

    for (int i = 0; i < n; i++)
    {
        int elem;
        cin >> elem;
        a.push_back(elem);
    }
    for (int i = 0; i < k; i++)
    {
        int elem;
        cin >> elem;
        b.push_back(elem);
    }

    int max_a = 0;
    vector<int> max_indexes;

    for (int i = 0; i < n; i++)
    {
        if (a[i] > max_a)
        {
            max_a = a[i];
        }
    }
    for (int i = 0; i < n; i++)
    {
        if (a[i] == max_a)
        {
            max_indexes.push_back(i + 1);
        }
    }

    bool result = false;
    for (int i = 0; i < k; i++)
    {

        for (int j = 0; j < max_indexes.size(); j++)
        {
            if (b[i] == max_indexes[j])
            {
                result = true;
            }
        }
    }

    if (result)
    {
        cout << "Yes" << endl;
    }
    else
    {
        cout << "No" << endl;
    }

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
