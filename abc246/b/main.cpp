#include <iostream>
#include <iomanip>
#include <cmath>
using namespace std;

int solveProblem()
{
    double a,b;
    cin >> a >> b;

    double x,y;
    double denom = sqrt(pow(a,2) + pow(b,2));

    x = a / denom;
    y = b / denom;

    cout << std::fixed;
    cout << std::setprecision(12) << x << " " << std::setprecision(12) << y << endl;

    return 0;
}

#ifndef TEST
int main(void)
{
    solveProblem();
}
#endif
