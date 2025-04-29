#include <iostream>
#include <vector>

using namespace std;

size_t order(vector<uint8_t> &array, size_t idx) {
  uint8_t order = 0;
  uint8_t base_num = array[idx];
  size_t i = idx + 1;
  while (i < array.size()) {
    if (array[i] < base_num) {
      ++order;
    }
    ++i;
  }
  return order;
}

int main() {
  size_t N, order_P = 0, order_Q = 0;
  vector<uint8_t> P, Q;
  vector<uint16_t> factorials;
  cin >> N;

  P.resize(N);
  Q.resize(N);
  factorials.resize(N);

  for (size_t i = 0; i < N; i++) {
    cin >> P[i];
  }

  for (size_t i = 0; i < N; i++) {
    cin >> Q[i];
  }

  factorials[0] = 1;
  for (size_t i = 1; i < N; i++) {
    factorials[i] = i * factorials[i-1];
  }

  for (size_t i = 0; i < N; i++) {
    order_P += order(P, N - 1 - i) * factorials[i];
    order_Q += order(Q, N - 1 - i) * factorials[i];
  }
  cout << (order_P < order_Q ? order_Q - order_P : order_P - order_Q) << endl;

  return 0;
}
