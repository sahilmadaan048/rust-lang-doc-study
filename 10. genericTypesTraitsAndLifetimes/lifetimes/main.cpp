#include "bits/stdc++.h"
using namespace std;

void solve()
{
    int* result;
    {
        int x = 10;
        result = &x; // result now points to x
    } // x goes out of scope here, result becomes a dangling pointer

    cout << result << endl; // Printing the dangling pointer address
    cout << *result << endl; // Undefined behavior: accessing a dangling pointer
}

int32_t main()
{
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    int T = 1;
    // cin >> T;
    while (T--)
    {
        solve();
    }
    return 0;
}
