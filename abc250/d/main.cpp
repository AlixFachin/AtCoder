#include <iostream>
using namespace std;
typedef long long ll;

#include <math.h>

#include <vector>

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

// -=-=- Logging functions -=-=-=-=-=-=
template <typename T>
void logVector(string vectorName, vector<T> *v)
{
#ifdef TEST
    cerr << "The content of " << vectorName << " is:" << endl;
    cerr << "[";
    if (v->size() > 0)
    {
        for (int i = 0; i < v->size() - 1; i++)
        {
            cerr << (*v)[i] << ",";
        }
        cerr << (*v)[(v->size() - 1)] << "]" << endl;
    }
    else
    {
        cerr << "]" << endl;
    }
#endif
}

void log(string s)
{
#ifdef TEST
    cerr << s << endl;
#endif
}

// returns true if p is prime "as far as we know"
bool isPrime(int p, vector<ll> primeList)
{
    for (int x : primeList)
    {
        if (p % x == 0)
        {
            return false;
        }
    }
    return true;
}

void buildPrimeList(int n, vector<ll> *primeList)
{
#define MAXP 1000005

    primeList->clear();

    vector<bool> primeMarker(MAXP, false);
    for (int i = 2; i < n; i++)
    {
        if (!primeMarker[i])
        {
            primeList->push_back(i);
            for (int j = i; j < MAXP; j += i)
            {
                primeMarker[j] = true;
            }
        }
    }
}

int solveProblem()
{
    ll n;
    cin >> n;
    ll result;

    vector<ll> primeList;

    // Let's get the easy cases out first
    if (n < 54)
    {
        result = 0;
    }
    else if (n < 250)
    {
        result = 1;
    }
    else if (n < 300)
    {
        int maxQ = round(pow(n, 1.0 / 3.0));
        vector<int> resultList;

        buildPrimeList(maxQ, &primeList);
        logVector("primeList", &primeList);

        // Now let's check the possibilities
        int q_index = 0, p_index;
        int nrPrimes = primeList.size();
        ll qCube = primeList[q_index] * primeList[q_index] * primeList[q_index];
        while (qCube < n && q_index < nrPrimes)
        {
            log("Checking : " + to_string(q_index) + "," + to_string(qCube));
            p_index = 0;
            while (p_index < q_index)
            {
                log("    Checking : " + to_string(p_index) + ',' + to_string(primeList[p_index]));
                if (primeList[p_index] * qCube <= n)
                {
                    resultList.push_back(primeList[p_index] * qCube);
                }
                else
                {
                    break;
                }
                p_index++;
            }
            q_index++;
            qCube = primeList[q_index] * primeList[q_index] * primeList[q_index];
        }

        logVector("similar number list:", &resultList);
        result = resultList.size();
    }
    else
    {
        log("Binary search ");
        // Binary search anyone?
        // int maxQ = round(pow(n, 1.0 / 3.0) / 2) + 1;
        int maxQ = 1000005;

        buildPrimeList(maxQ, &primeList);
        logVector("primes", &primeList);

        // Now let's check the possibilities

        int q_index = 1, p_index, min_index;
        int nrPrimes = primeList.size();
        ll resultCount = 0;
        bool not_found = true;
        ll qCube = primeList[q_index] * primeList[q_index] * primeList[q_index];
        while (qCube < n && q_index < nrPrimes)
        {
            log("Checking : " + to_string(q_index) + "," + to_string(primeList[q_index]) + ",^3=" + to_string(qCube));

            if (primeList[q_index - 1] * qCube <= n)
            {
                // Then they all have to be added
                resultCount += q_index;
            }
            else
            {
                // Check if there are *any* solutions for the problem with q
                if (primeList[0] * qCube < n)
                {
                    p_index = q_index / 2;
                    min_index = 0;
                    not_found = true;
                    while (p_index < q_index && not_found)
                    {
                        log("--Checking : " + to_string(p_index) + "," + to_string(q_index));
                        if (primeList[p_index] * qCube <= n && primeList[p_index + 1] * qCube > n)
                        {
                            not_found = false;
                            resultCount += p_index + 1;
                        }
                        else
                        {
                            // Not at the limit -> will need to find out

                            if (primeList[p_index] * qCube > n)
                            {
                                if (p_index - min_index < 10)
                                {
                                    p_index--;
                                }
                                else
                                {
                                    p_index = (p_index + min_index) / 2;
                                }
                            }
                            else
                            {
                                min_index = p_index;
                                if (q_index - p_index < 10)
                                {
                                    p_index++;
                                }
                                else
                                {
                                    p_index = (p_index + q_index) / 2;
                                }
                            }
                        }
                    }
                }
            }

            q_index++;
            qCube = primeList[q_index] * primeList[q_index] * primeList[q_index];
        }

        result = resultCount;
    }

    cout << result << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
