#include <iostream>
using namespace std;

#include <array>
#include <vector>

// Some hints:
// #include <algorithm> for sort, sort(beginning_iterator, end_iterator)
//
// #include <iomanip> for setPrecision and fixed
// cin >> x >> y >> z
// cout << "This is output"

int solveProblem()
{
    int N, M, K, S, T, X;
    const long long modulo = 998244353LL;
    cin >> N >> M >> K >> S >> T >> X;
    S--;
    T--;
    X--;
    vector<pair<int, int>> edge(M);
    for (auto &[U, V] : edge)
    {
        cin >> U >> V;
        U--;
        V--;
    }

    vector dp(K + 1, vector(N, array<long long, 2>{0, 0}));
    dp[0][S][0] = 1;
    for (int i = 0; i < K; i++)
    {
        for (auto [U, V] : edge)
            for (int x : {0, 1})
            {
                dp[i + 1][V][x ^ (V == X)] += dp[i][U][x] % modulo;
                dp[i + 1][U][x ^ (U == X)] += dp[i][V][x] % modulo;
            }
    }

    cout << dp[K][T][0] << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
