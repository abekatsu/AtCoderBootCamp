use std::collections::HashSet;
use proconio::input;

// https://atcoder.jp/contests/abc071/tasks/abc071_b
// 英小文字からなる文字列 S が与えられます．
// S に現れない英小文字であって，最も辞書順（アルファベット順）で小さいものを求めてください．
// ただし，S にすべての英小文字が現れる場合は，代わりに None を出力してください．

fn main() {
    input! {
        s: String,
    }

    let result: HashSet<char> = s.chars().collect();

    match ('a'..='z').find(|ch| !result.contains(ch)) {
        Some(ch) => println!("{}", ch),
        None => println!("None"),
    }

}
