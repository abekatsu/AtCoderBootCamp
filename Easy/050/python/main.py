# https://atcoder.jp/contests/abc059/tasks/abc059_b
# 2 つの正整数 A,B が与えられるので、その大小を比較してください。
# ただし A, B のとり得る値は1以上 10^100 以下とします。
# A>B のときGREATER、A<B のときLESS、A=B のときEQUALと出力せよ。

num = [0, 0]
num[0] = str(input())
num[1] = str(input())

if len(num[0]) < len(num[1]):
    print("LESS")
elif len(num[0]) > len(num[1]):
    print("GREATER")
else:
    isEqual = True
    for idx in range(len(num[0])):
        if int(num[0][idx]) < int(num[1][idx]):
            print("LESS")
            isEqual = False
            break
        elif int(num[0][idx]) > int(num[1][idx]):
            print("GREATER")
            isEqual = False
            break

    if isEqual:
        print("EQUAL")
