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

string findPath(int start, int end, unordered_map<int, vector<int>> neighbors, unordered_set<int> visited, string pathSoFar)
{
    vector<int> start_neighbours = neighbors[start];
    string reply = "";
    for (const auto &neigh : start_neighbours)
    {
        // best case: we arrive at the end
        if (neigh == end)
        {
            return (pathSoFar + to_string(end));
        }
        // else, if the neighbor has already been visited
        if (visited.count(neigh))
        {
            // let's look at next node
            continue;
        }
        // not visited yet - so we try it
        visited.insert(neigh);
        string s = findPath(neigh, end, neighbors, visited, pathSoFar + to_string(neigh) + " ");
        reply += s;
    }
    return reply;
}

int solveProblem()
{
    int n, x, y;
    cin >> n >> x >> y;

    vector<pair<int, int>> edges;
    unordered_map<int, vector<int>> neighbors;
    for (int i = 1; i < n; i++)
    {
        int a, b;
        cin >> a >> b;
        edges.push_back({a, b});
        if (!neighbors.count(a))
        {
            neighbors[a] = {b};
        }
        else
        {
            neighbors[a].push_back(b);
        }
        if (!neighbors.count(b))
        {
            neighbors[b] = {a};
        }
        else
        {
            neighbors[b].push_back(a);
        }
    }
    string path = findPath(x, y, neighbors, {}, to_string(x) + " ");

    cout << path << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
