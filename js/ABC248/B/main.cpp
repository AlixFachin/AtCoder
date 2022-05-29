#include <iostream>
#include <algorithm>
#include <math.h>

using namespace std;
typedef long long ll;

ll a, b, k;
ll result;

int main(void) {
    cin >> a >> b >> k;

    if (a == b) {
        cout << '0' << endl;
        return 0;
    }

    result = log(b/a)/log(k);
    cout << result << endl;

}


