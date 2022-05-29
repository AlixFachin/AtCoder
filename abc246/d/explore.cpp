#include <iostream>
using namespace std;

typedef long long ll;

ll n;

ll f(ll a, ll b, ll n)
{
    return a * a * a + a * a * b + a * b * b + b * b * b - n;
}

struct point
{
    ll x;
    ll y;
    ll value;
};

point setPoint(ll x, ll y)
{
    point res;

    res.x = x;
    res.y = y;
    res.value = f(x, y, n);

    return res;
}

int main(void)
{

    cout << "Please input n \n";
    cin >> n;

    bool quit = false;
    point a, b, c, d, m;

    a = setPoint(0, n);
    b = setPoint(n, n);
    c = setPoint(0, 0);
    d = setPoint(n, 0);
    m = setPoint(n / 2, n / 2);

    while (!quit)
    {
        string choice;

        cout << "======================================" << endl;
        cout << " A --- B" << endl;
        cout << " |  M  |" << endl;
        cout << " C --- D" << endl;
        cout << "A: " << a.x << "," << a.y << ":" << a.value << endl;
        cout << "B: " << b.x << "," << b.y << ":" << b.value << endl;
        cout << "C: " << c.x << "," << c.y << ":" << c.value << endl;
        cout << "D: " << d.x << "," << d.y << ":" << d.value << endl;
        cout << "M: " << m.x << "," << m.y << ":" << m.value << endl;

        cout << "Please input your choice (Q for Quit)" << endl;
        cin >> choice;

        if (choice == "Q")
        {
            quit = true;
        }
        else if (choice == "A")
        {
            b = setPoint((a.x + b.x) / 2, b.y);
            c = setPoint(a.x, (a.y + c.y) / 2);
            d = setPoint(m.x, m.y);
            m = setPoint((a.x + b.x) / 2, (a.y + c.y) / 2);
        }
        else if (choice == "C")
        {
            a = setPoint(a.x, (a.y + c.y) / 2);
            b = setPoint(m.x, m.y);
            d = setPoint((c.x + d.x) / 2, d.y);
            m = setPoint((a.x + b.x) / 2, (a.y + c.y) / 2);
        }
        else if (choice == "B")
        {
            a = setPoint((a.x + b.x) / 2, a.y);
            c = setPoint(m.x, m.y);
            d = setPoint(d.x, (b.y + d.y) / 2);
            m = setPoint((a.x + b.x) / 2, (a.y + c.y) / 2);
        }
    }

    return 0;
}
