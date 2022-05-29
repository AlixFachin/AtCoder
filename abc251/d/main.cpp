#include <iostream>
#include <vector>        // for the weight list
#include <unordered_set> // for the numbers we can build

typedef long long ll;
using namespace std;

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

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

int solveProblem()
{
    int w;
    ll max_element = 1;
    ll element;

    cin >> w;

    unordered_set<int> weights_flag;
    vector<int> weights;
    weights_flag.insert(1);
    weights.push_back(1);

    unordered_set<ll> results;
    results.insert(1);

    for (int i = 1; i <= w; i++)
    {
        log("Looking at " + to_string(i));
        logVector("weights", &weights);
        logSet("results", &results);

        if (!results.count(i))
        {
            // we didn't have the result. We need to add it.
            if (weights.size() <= 300)
            {
                log("Did not find " + to_string(i));
                // Algorithm 1 -> we add the max
                if (weights_flag.count(i - 1))
                {
                    element = i;
                }
                else
                {
                    element = i - 1;
                }
                if (element > max_element)
                {
                    max_element = element;
                }
                cerr << "Adding " << element << endl;
                results.insert(element);
                for (int i = 0; i < weights.size(); i++)
                {
                    results.insert(weights[i] + element);
                    for (int j = i + 1; j < weights.size(); j++)
                    {
                        results.insert(weights[i] + weights[j] + element);
                    }
                }
                weights.push_back(element);
                weights_flag.insert(element);

                logVector("weights", &weights);
                logSet("results", &results);
            }
        }
    }

    cout << weights.size() << endl;
    for (int i = 0; i < weights.size() - 1; i++)
    {
        cout << weights[i] << " ";
    }
    cout << weights[weights.size() - 1] << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
