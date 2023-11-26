#include <iostream>
using namespace std;

#include <vector>
#include <unordered_set>
#include <math.h>
#include <iomanip>

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

void plog(string s)
{
#ifdef TEST
    cerr << s << endl;
#endif
}
// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

int solveProblem()
{
    int n, k;
    cin >> n >> k;

    vector<int> lampArray;

    for (int i = 0; i < k; i++)
    {
        int a;
        cin >> a;
        lampArray.push_back(a - 1);
    }

    vector<pair<int, int>> persons;
    for (int i = 0; i < n; i++)
    {
        int x, y;
        cin >> x >> y;
        persons.push_back({x, y});
    }

    logVector("lamp", &lampArray);

    double max_dist = -1;
    // For each person, we will look at the distance to the closest light
    for (int i = 0; i < n; i++)
    {
        double min_dist = MAXFLOAT;
        auto [x, y] = persons[i];
        for (int j = 0; j < k; j++)
        {
            int lampRef = lampArray[j];
            auto [lx, ly] = persons[lampRef];
            double dist = 1.0 * (ly - y) * (ly - y) + 1.0 * (lx - x) * (lx - x);
            if (dist <= min_dist)
            {
                min_dist = dist;
            }
            plog("Checking dist btw (" + to_string(x) + "," + to_string(y) + ") and (" + to_string(lx) + "," + to_string(ly) + ") - dist=" + to_string(dist) + ",minDist=" + to_string(min_dist));
        }
        // min_dist will contain the minimum distance to all the light sources
        if (min_dist > max_dist)
        {
            max_dist = min_dist;
        }
    }

    cout << std::setprecision(12) << sqrtl(max_dist) << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
