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
    int n;
    vector<string> reels;

    cin >> n;
    for (int i = 0; i < n; i++)
    {
        string s;
        cin >> s;
        reels.push_back(s);
    }

    logVector("reels", &reels);

    vector<int> result;
    // Main Search ->
    for (int i = 0; i < 10; i++)
    {
        unordered_set<int> pushed_at_time;
        char char_i = to_string(i)[0];
        int count_spin = 0;
        log("Looking at getting the " + to_string(i));
        for (int j = 0; j < n; j++)
        {
            string reel;
            reel = reels[j];
            // finding occurence of i in string s;
            auto index = reel.find(char_i);
            while (pushed_at_time.count(index))
            {
                index += 10;
            }
            log("  Reel " + to_string(j) + " -> " + to_string(index));
            pushed_at_time.insert(index);
            count_spin = index > count_spin ? index : count_spin;
        }
        log("Spin for " + to_string(i) + " is " + to_string(count_spin));
        result.push_back(count_spin);
    }

    int min_spin = 100000;
    for (int i = 0; i < result.size(); i++)
    {
        if (result[i] < min_spin)
        {
            min_spin = result[i];
        }
    }

    cout << min_spin << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
