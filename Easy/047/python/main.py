# https://atcoder.jp/contests/abc079/tasks/abc079_b
# 今、日本は 11 月 18 日ですが、11 と 18 は隣り合うリュカ数です。
# 整数 N が与えられるので、N 番目のリュカ数を求めてください。
# ただし、リュカ数は i 番目のリュカ数を Li​ とすると、
#   L0​=2
#   L1​=1
#   Li​= Li−1​+Li−2​ (i≧2)
# と定義される数とします。

n = int(input())
lucas = [2, 1]

for idx in range(2, n + 1):
    num = lucas[idx - 2] + lucas[idx - 1]
    lucas.append(num)

print(lucas[n])
