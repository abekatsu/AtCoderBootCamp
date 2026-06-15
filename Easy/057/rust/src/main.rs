// https://atcoder.jp/contests/abc158/tasks/abc158_c
// 問題文

// 消費税率が 8 %のとき A 円、10 ％のとき B 円の消費税が課されるような商品の税抜き価格を求めてください。
// ただし、税抜き価格は正の整数でなければならないものとし、消費税の計算において小数点以下は切り捨てて計算するものとします。
// 条件を満たす税抜き価格が複数存在する場合は最も小さい金額を出力してください。また、条件を満たす税抜き価格が存在しない場合は -1 と出力してください。
// 制約
//     1≤A≤B≤100
//     A,B は整数である
// 入力
// 入力は以下の形式で標準入力から与えられる。
// A B
// 出力
// 条件を満たす税抜き価格が存在する場合は最小の金額を表す整数を、存在しない場合は -1 を出力せよ。

use proconio::input;

// percent % の消費税がちょうど tax 円になる最小の税抜き価格を返す
fn possible_ex_tax(tax: u16, percent: u16) -> u16 {
    let mut price = tax * 100 / percent;
    loop {
        if price * percent / 100 == tax {
            return price;
        }
        price += 1;
    }
}

fn ex_tax(a: u16, b: u16) -> i16 {
    // 8% で a 円、10% で b 円になる最小の税抜き価格の候補
    let candidate = possible_ex_tax(a, 8).max(possible_ex_tax(b, 10));

    // 候補が両方の条件を満たすか確認する（満たさなければ解なし）
    if candidate * 8 / 100 == a && candidate * 10 / 100 == b {
        candidate as i16
    } else {
        -1
    }
}

fn main() {
    input! {
        a: u16,
        b: u16,
    }

    println!("{}", ex_tax(a, b));
}

#[cfg(test)]
mod tests {
    use super::ex_tax;
    #[test]
    fn samples() {
        assert_eq!(ex_tax(2, 2), 25);
        assert_eq!(ex_tax(8, 10), 100);
        assert_eq!(ex_tax(19, 99), -1);
    }
}
