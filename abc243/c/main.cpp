#include <iostream>
using namespace std;

#include <vector>
#include <unordered_set>
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
    vector<pair<int, int>> coord;

    // Table will contain the min X for people walking left, and the max X for people walking right
    unordered_map<int, int> left_max_list;
    unordered_map<int, int> right_min_list;

    cin >> n;

    for (int i = 0; i < n; i++)
    {
        int x, y;
        cin >> x >> y;
        coord.push_back({x, y});
    }

    string s;
    cin >> s;

    for (int i = 0; i < n; i++)
    {
        char direction = s[i];
        auto [x, y] = coord[i];
        if (direction == 'L')
        {
            // getting the min of the ppl at the same row
            if (left_max_list.count(y))
            {
                left_max_list[y] = max(left_max_list[y], x);
            }
            else
            {
                left_max_list[y] = x;
            }
        }
        else
        {
            if (right_min_list.count(y))
            {
                right_min_list[y] = min(right_min_list[y], x);
            }
            else
            {
                right_min_list[y] = x;
            }
        }
    }

    // Iterating the list (whichever is fine)
    for (auto left_data : left_max_list)
    {
        int y = left_data.first;
        int max_x = left_data.second;
        if (right_min_list.count(y))
        {
            int min_x = right_min_list[y];
            if (max_x >= min_x)
            {
                cout << "Yes" << endl;
                return 0;
            }
        }
    }

    cout << "No" << endl;
    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
