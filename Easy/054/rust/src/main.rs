// https://atcoder.jp/contests/abc109/tasks/abc109_b
// B. Shiritori
// 問題文
// 高橋くんは今日も 1 人でしりとりの練習をしています。
// しりとりとは以下のルールで遊ばれるゲームです。
// ・ はじめ、好きな単語を発言する
//    以降、次の条件を満たす単語を発言することを繰り返す
//    ・その単語はまだ発言していない単語である
//    ・その単語の先頭の文字は直前に発言した単語の末尾の文字と一致する
// 高橋くんは、10 秒間にできるだけ多くの単語を発言する練習をしています。
// 高橋くんが発言した単語の個数 N と i 番目に発言した単語 Wi​ が与えられるので、どの発言もしりとりのルールを守っていたかを判定してください。
// 制約
//     N は 2≤N≤100 を満たす整数である
//     Wi​ は英小文字からなる長さ 1 以上 10 以下の文字列である
use proconio::input;
use std::collections::HashSet;

fn is_shiritoried(words: &[String]) -> bool {
    let mut last_char: Option<char> = None;
    let mut used_words: HashSet<&str> = HashSet::new();

    for word in words.iter() {
        if !used_words.insert(word.as_str()) {
            return false;
        }

        let first = match word.chars().next() {
            Some(c) => c,
            None => return false,
        };

        if let Some(prev) = last_char {
            if prev != first {
                return false;
            }
        }
        last_char = word.chars().last();
    }
    true
}

fn main() {
    input! {
        n: usize,
        words: [String; n],
    }

    println!("{}", if is_shiritoried(&words) { "Yes" } else { "No" });
}

#[cfg(test)]
mod tests {
    use super::is_shiritoried;

    #[test]
    fn samples() {
        assert_eq!(
            is_shiritoried(&vec![
                String::from("hoge"),
                String::from("english"),
                String::from("hoge"),
                String::from("enigma")
            ]),
            false
        );
    }
}
