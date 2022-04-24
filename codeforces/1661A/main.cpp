#include <iostream>
#include <vector>

using namespace std;

unsigned long long min_sum(const vector<int>& a, const vector<int>& b) {
    int first_elem1 = 0;
    int first_elem2 = 0;
    if (a.size() > 0) {
        first_elem1 = a[0];
        first_elem2 = b[0];
    }
    unsigned long long res_sum = 0;
    for (size_t i = 0; i < a.size() - 1; ++i ) {
        int sum1 = std::abs(first_elem1- a[i + 1]) + std::abs(first_elem2 - b[i + 1]);
        int sum2 = std::abs(first_elem1 - b[i + 1]) + std::abs(first_elem2 - a[i + 1]);
        if (sum1 < sum2) {
            res_sum += sum1;
            first_elem1 = a[i + 1];
            first_elem2 = b[i + 1];
        } else {
            res_sum += sum2;
            first_elem1 = b[i + 1];
            first_elem2 = a[i + 1];
        }
    }
    return res_sum;
}

int main() {
    size_t n, t, elem;
    vector<int> a;
    vector<int> b;
    cin >> t;
    for (int i = 0; i < t; ++i) {
        cin >> n;
        a.clear();
        b.clear();
        for (size_t j = 0; j < n; ++j) {
            cin >> elem;
            a.push_back(elem);
        }
        for (size_t j = 0; j < n; ++j) {
            cin >> elem;
            b.push_back(elem);
        }
        auto res = min_sum(a, b);
        cout<< res << "\n";
    }
    return 0;
}
