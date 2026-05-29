// https://atcoder.jp/contests/abc134/tasks/abc134_c
// 長さ N の数列 A1​,A2​,...,AN​ が与えられます。 1 以上 N 以下の各整数 i に対し、次の問いに答えてください。
// 数列中の Ai​ を除く N−1 個の要素のうちの最大の値を求めよ。

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut max_value: u32 = 0;
    let mut second_value: u32 = 0;

    a.iter().for_each({|&v| {
        if v > max_value {
            second_value = max_value;
            max_value = v;
        } else if v > second_value {
            second_value = v;
        }
    }});

    a.iter().for_each({|&v| {
        if v < max_value {
            println!("{}", max_value);
        } else {
            println!("{}", second_value);
        }
    }});

}
