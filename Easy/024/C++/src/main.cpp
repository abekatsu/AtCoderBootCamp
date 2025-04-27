#include <iostream>
#include <cstdint>
#include <vector>

using namespace std;

int main() {
  size_t A, B, M;
  vector<int> a, b;
  size_t min_a = SIZE_MAX, min_b = SIZE_MAX, min = SIZE_MAX;

  cin >> A >> B >> M;

  a.resize(A);
  b.resize(B);

  for (size_t i = 0; i < A; i++) {
    cin >> a[i];
    if (a[i] < min_a) {
      min_a = a[i];
    }
  }

  for (size_t i = 0; i < B; i++) {
    cin >> b[i];
    if (b[i] < min_b) {
      min_b = b[i];
    }
  }

  min = min_a + min_b;

  for (size_t i = 0; i < M; i++) {
    size_t x, y, c;
    cin >> x >> y >> c;

    size_t price = a[x-1] + b[y-1] - c;
    if (price < min) {
      min = price;
    }
  }

  cout << min << endl;
  return 0;
}
