#include <iostream>
using namespace std;

#include <vector>
#include <unordered_map>
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

struct Edge
{
    int toNode;
    int weight;
    int id;
};

class Graph
{
private:
    int nrNodes;
    int nrEdges;
    unordered_map<int, vector<Edge>> adjacentList;
    unordered_set<int> nodeList;

public:
    Graph(int numNodes, int numEdges)
    {
        nrNodes = numNodes;
        nrEdges = numEdges;
    }

    string to_string()
    {
        string result = "";
        for (const auto &key_value : adjacentList)
        {
            result += "From " + std::to_string(key_value.first) + " to: {";
            for (const Edge &edge : key_value.second)
            {
                result += std::to_string(edge.toNode);
                result += ": ";
                result += std::to_string(edge.weight);
                result += ",";
            }
            result += "}\n";
        }
        return result;
    }

    void addEdge(int fromNode, int toNode, int weight, int edgeId)
    {
        if (!adjacentList.count(fromNode))
        {
            adjacentList[fromNode] = {};
        }
        if (!adjacentList.count(toNode))
        {
            adjacentList[toNode] = {};
        }
        Edge newEdge;
        newEdge.toNode = toNode;
        newEdge.weight = weight;
        newEdge.id = edgeId;
        Edge reverseEdge;
        reverseEdge.toNode = fromNode;
        reverseEdge.weight = weight;
        reverseEdge.id = edgeId;

        adjacentList[fromNode].push_back(newEdge);
        adjacentList[toNode].push_back(reverseEdge);

        nodeList.insert(fromNode);
        nodeList.insert(toNode);
    }

    /*
     * djikstra function returns a vector with the shortest routes
     */
    vector<int> djikstra(int startNode)
    {
        struct data
        {
            bool visited;
            ll distance;
            int previous;
            int edgeId;
        };

        unordered_map<int, data> algoData;
        vector<int> bestEdges; // for the result at the end

        // Initialisation
        for (const int &node : nodeList)
        {
            algoData[node].visited = false;
            algoData[node].distance = node == startNode ? 0 : INT64_MAX;
            algoData[node].previous = -1;
            algoData[node].edgeId = -1;
        }

        using P = pair<ll, int>;
        priority_queue<P, vector<P>, greater<P>> pq;
        pq.push({0, startNode});

        while (!pq.empty())
        {
            auto [d, bestNode] = pq.top();
            pq.pop();

            // We don't do anything if we already visited the node
            if (algoData[bestNode].distance != d)
                continue;

            // We are at "bestNode" - we need to examine all the unvisited neighbours

            for (auto [neighbour, c, idx] : adjacentList[bestNode])
            {
                // For each neighbour, we compute the distance from the starting point (distance from current node + edge weight)
                // If the distance is shorter than the one we already have in store, we update the distance and the previous node
                if (algoData[neighbour].distance > algoData[bestNode].distance + c)
                {
                    algoData[neighbour].distance = algoData[bestNode].distance + c;
                    algoData[neighbour].edgeId = idx;
                    algoData[neighbour].previous = bestNode;
                    pq.push({algoData[neighbour].distance, neighbour});
                }
            }
        }

        // We have to return an array with the shortest path
        for (const int &node : nodeList)
        {
            if (node != startNode && algoData[node].edgeId != -1)
            {
                bestEdges.push_back(algoData[node].edgeId);
            }
        }

        return bestEdges;
    }
};

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

int solveProblem()
{

    int n, m;
    vector<int> bestEdges;

    cin >> n >> m;
    Graph roads(n, m);

    for (int i = 0; i < m; i++)
    {
        int a, b, c;

        cin >> a >> b >> c;

        roads.addEdge(a, b, c, i + 1);
    }

    cerr << roads.to_string();

    bestEdges = roads.djikstra(1);

    for (int i = 0; i < bestEdges.size(); i++)
    {
        cout << bestEdges[i] << (i == bestEdges.size() - 1 ? "" : " ");
    }

    cout << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
