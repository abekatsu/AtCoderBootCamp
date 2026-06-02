// https://atcoder.jp/contests/abc044/tasks/abc044_b
// w を、英小文字のみからなる文字列とします。 w が以下の条件を満たすならば、w を美しい文字列と呼ぶことにします。
//   どの英小文字も、w 中に偶数回出現する。
// 文字列 w が与えられます。w が美しい文字列かどうか判定してください。

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut chars = vec![0; 26];

    for c in s.chars() {
        chars[(c as u8 - b'a') as usize] += 1;
    }

    let result = chars.iter().all(|v| v % 2 == 0);
    println!("{}", if result { "Yes" } else { "No" });
}
