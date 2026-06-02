// https://atcoder.jp/contests/abc044/tasks/abc044_b
// w を、英小文字のみからなる文字列とします。 w が以下の条件を満たすならば、w を美しい文字列と呼ぶことにします。
//   どの英小文字も、w 中に偶数回出現する。
// 文字列 w が与えられます。w が美しい文字列かどうか判定してください。

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mask = s.bytes().fold(0u32, |mask, c| mask ^ (1 << (c - b'a')));

    println!("{}", if mask == 0 { "Yes" } else { "No" });
}
