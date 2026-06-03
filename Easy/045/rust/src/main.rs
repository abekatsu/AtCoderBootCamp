// https://atcoder.jp/contests/abc090/tasks/abc090_b
// A 以上 B 以下の整数のうち、回文数となるものの個数を求めてください。
// ただし、回文数とは、先頭に 0 をつけない 10 進表記を文字列として見たとき、前から読んでも後ろから読んでも同じ文字列となるような正の整数のことを指します。
// 10000 ≤ A ≤ B ≤ 99999
// 11009 11332
// 11332 - 11009 = 323
// 31415 92653
// 92653 - 31415 = 61238
use proconio::input;

// X 以下の 5 桁回文数の個数を返す。
// 5 桁回文数は先頭 3 桁 (prefix, 100..=999) で一意に決まるので、
// 「palindrome(prefix) <= X を満たす prefix の個数」を数えればよい。
fn count_le(x: i32) -> i32 {
    if x < 10000 {
        return 0;
    }
    let pre = x / 100; // 先頭 3 桁
    let (d1, d2, d3) = (pre / 100, pre / 10 % 10, pre % 10);
    let pal = d1 * 10000 + d2 * 1000 + d3 * 100 + d2 * 10 + d1;
    if pal <= x {
        pre - 99
    } else {
        pre - 100
    }
}

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    println!("{}", count_le(b) - count_le(a - 1));
}
