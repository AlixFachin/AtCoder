#include <iostream>
using namespace std;
#include <vector>
#include <unordered_map>
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

void logMap(string mapName, unordered_map<ll, ll> m)
{
    cerr << "The content of " << mapName << "is:" << endl;
    for (auto &elem : m)
    {
        cerr << " " << to_string(elem.first) << "," << to_string(elem.second) << endl;
    }
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
    int q;
    cin >> q;

    unordered_map<ll, ll> container;
    // const long MAX_SIZE = 1E9 + 1;
    // ll counts[MAX_SIZE];
    bool onePrint = false;

    ll tempMin = 1E10;
    ll tempMax = -1;
    bool shouldCompute = false;

    for (int i = 0; i < q; i++)
    {
        int queryType;
        cin >> queryType;
        if (queryType == 1)
        {
            ll elem;
            cin >> elem;
            // inserting into the container

            // counts[elem] += 1;
            if (container.count(elem))
            {
                container[elem] += 1;
            }
            else
            {
                container[elem] = 1;
            }

            if (elem > tempMax)
            {
                tempMax = elem;
            }
            if (elem < tempMin)
            {
                tempMin = elem;
            }

            // log("Insert " + to_string(elem));
            // logMap("c", container);
        }
        else if (queryType == 2)
        {
            ll elem;
            ll c;
            cin >> elem >> c;

            if (container.count(elem))
            {
                if (c >= container[elem])
                {
                    if (elem == tempMax || elem == tempMin)
                    {
                        // we removed min or max, need to recompute
                        shouldCompute = true;
                    }
                }
                container[elem] -= min(c, container[elem]);
            }

            // log("Remove " + to_string(elem));
            // logMap("c", container);
        }
        else
        {
            // need to output the min
            ll cmin = 1E10;
            ll cmax = -1;
            if (shouldCompute)
            {
                for (const auto &key_value : container)
                {
                    if (key_value.second != 0)
                    {
                        if (key_value.first < cmin)
                        {
                            cmin = key_value.first;
                        }
                        if (key_value.first > cmax)
                        {
                            cmax = key_value.first;
                        }
                    }
                }
                /*int i = 0;
                while (counts[i] == 0)
                {
                    i++;
                }
                cmin = i;
                i = MAX_SIZE;
                while (counts[i] == 0)
                {
                    i--;
                }
                cmax = i;*/

                tempMax = cmax;
                tempMin = cmin;
                shouldCompute = false;
            }

            // log("Printout values " + to_string(cMin->second) + "," + to_string(cMax->second));
            cout << (tempMax - tempMin) << endl;
            onePrint = true;
        }
    }

    if (!onePrint)
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
