// https://atcoder.jp/contests/abc090/tasks/abc090_b
// A 以上 B 以下の整数のうち、回文数となるものの個数を求めてください。
// ただし、回文数とは、先頭に 0 をつけない 10 進表記を文字列として見たとき、前から読んでも後ろから読んでも同じ文字列となるような正の整数のことを指します。
// 10000 ≤ A ≤ B ≤ 99999
// 11009 11332
// 11332 - 11009 = 323
// 31415 92653
// 92653 - 31415 = 61238
use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    // let mut result = (b - a) / 100;
    // if ((b / 1000) - (b % 100)) <= 0 {
    //     result += 1;
    // }

    let mut result = 0;
    for num in a..=b {
        if (num / 10000 == num % 10) && ((num / 1000) % 10 == (num % 100) / 10) {
            result += 1;
        }
    }
    println!("{}", result);
}
