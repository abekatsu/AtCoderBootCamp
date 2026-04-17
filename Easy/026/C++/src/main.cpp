#include <iostream>

using namespace std;

long attack_count(size_t hp) {
  // cout << "HP: " << hp << endl;
  if (hp == 1)
    return 1;

  return 1 + (2 * attack_count(hp/2)); // 分裂するのに1回。半分のHPのモンスターを倒すために必要な回数 * 2
}

int main() {
  size_t H;
  cin >> H;

  cout << attack_count(H) << endl;
}
