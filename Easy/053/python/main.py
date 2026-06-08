# https://atcoder.jp/contests/abc061/tasks/abc061_b
# B - Counting Roads

# 実行時間制限: 2 sec / メモリ制限: 256 MiB

# 配点 : 200 点
# 問題文

# N 個の都市があり、M 本の道路があります。
# i(1≦i≦M) 番目の道路は、都市 ai​ と 都市 bi​ (1≦ai​,bi​≦N) を双方向に結んでいます。
# 同じ 2 つの都市を結ぶ道路は、1 本とは限りません。
# 各都市から他の都市に向けて、何本の道路が伸びているか求めてください。
# 制約

#     2≦N,M≦50
#     1≦ai​,bi​≦N
#     ai​ \not= bi​
#     入力は全て整数である。

# 入力

# 入力は以下の形式で標準入力から与えられる。

# N M
# a1​ b1​
# :
# aM​ bM​

# 出力

# 答えを N 行に出力せよ。
# i(1≦i≦N) 行目には、都市 i から他の都市に向けて、何本の道路が伸びているかを出力せよ。

n, m = map(int, input().split(" "))

roads = [[0] * n for i in range(n)]

for i in range(m):
    a, b = map(int, input().split(" "))
    roads[a - 1][b - 1] += 1
    roads[b - 1][a - 1] += 1


for i in range(n):
    num = 0
    for v in roads[i]:
        num += v
    print(num)
