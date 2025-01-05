#include <algorithm>
#include <numeric>
#include <iostream>
#include <vector>

using namespace std;
size_t maxNumberDelightChildren(size_t N, int x, vector<int> & vec) {
  size_t result = 0;
  std::sort(vec.begin(), vec.end());
  int sum = 0;
  for (size_t i = 0; i < vec.size(); i++) {
    sum += vec[i];
    if (sum > x) {
      break;
    }
    result += 1;
  }
  if (result == N && sum < x) {
    result -= 1;
  }
  return result;
}

int main() {
  size_t N, x;
  vector<int> a;

  cin >> N >> x;
  a.resize(N);
  for (size_t i = 0; i < N; i++) {
    cin >> a[i];
  }

  cout << maxNumberDelightChildren(N, x, a) << endl;
}
