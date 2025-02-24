#include <iostream>
#include <vector>

using namespace std;

size_t number_of_chocolates(size_t N, size_t D, size_t X, vector<int> &a) {
  size_t num_of_chocolates = 0;

  for (size_t i = 0; i < N; i++) {
    num_of_chocolates += (D - 1) / a[i] + 1;
  }

  return num_of_chocolates + X;
}

int main() {
  size_t N, D, X;
  vector<int> a;

  cin >> N;
  cin >> D >> X;
  a.resize(N);
  for (size_t i = 0; i < N; i++) {
    cin >> a[i];
  }
  cout << number_of_chocolates(N, D, X, a) << endl;

  return 0;
}
