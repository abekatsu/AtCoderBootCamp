# https://atcoder.jp/contests/abc093/tasks/abc093_b
# B - Small and Large Integer
# 問題文
# 以下を満たす整数をすべて昇順に出力してください。
#     A 以上 B 以下の整数の中で、小さい方から K 番目以内であるか、大きい方から K 番目以内である
# 制約
#     1≤A≤B≤10^9
#     1≤K≤100
#     入力はすべて整数である
# 入力
# 入力は以下の形式で標準入力から与えられる。
# A B K
# 出力
# 上の条件を満たす整数をすべて昇順に出力せよ。

a, b, k = map(int, input().split(" "))

for i in range(a, min(a + k - 1, b) + 1):
    print(i)

for i in range(max(a + k, b - k + 1), b + 1):
    print(i)
