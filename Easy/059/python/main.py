# https://atcoder.jp/contests/abc104/tasks/abc104_b
# B - AcCepted
# 実行時間制限: 2 sec / メモリ制限: 1024 MiB

# 配点 : 200 点
# 問題文

# 文字列 S が与えられます。S のそれぞれの文字は英大文字または英小文字です。 S が次の条件すべてを満たすか判定してください。
#     S の先頭の文字は大文字の A である。
#     S の先頭から 3 文字目と末尾から 2 文字目の間（両端含む）に大文字の C がちょうど 1 個含まれる。
#     以上の A, C を除く S のすべての文字は小文字である。

# 制約
#     4≤∣S∣≤10 （∣S∣ は文字列 S の長さ）
#     S のそれぞれの文字は英大文字または英小文字である。

# 入力
# 入力は以下の形式で標準入力から与えられる。
# S

# 出力
# S が問題文中の条件すべてを満たすなら AC、そうでなければ WA と出力せよ。


def AcCepted(s):
    includeC = False
    for idx, char in enumerate(s):
        if idx == 0:
            if char != "A":
                return False
        else:
            if idx >= 2 and idx <= len(s) - 2:
                if char == "C" and (not includeC):
                    includeC = True
                elif char >= "A" and char <= "Z":
                    return False
            elif char >= "A" and char <= "Z":
                return False

    return includeC


s = input()
print("AC" if AcCepted(s) else "WA")
