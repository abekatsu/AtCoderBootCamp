// https://atcoder.jp/contests/abc134/tasks/abc134_c
// 長さ N の数列 A1​,A2​,...,AN​ が与えられます。 1 以上 N 以下の各整数 i に対し、次の問いに答えてください。
// 数列中の Ai​ を除く N−1 個の要素のうちの最大の値を求めよ。

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut left = vec![0u32; n+1];
    let mut right = vec![0u32; n+1];

    // i番目に対して、左右の累積和を求める。
    // left[i+1] には a[0] から a[i] までの最大値 (i.e. left[i]とa[i]の大きい方を代入する) を代入する
    // right[i+1] には a[i] から a[n-1] までの最大値を代入する
    for (idx, _v) in a.iter().enumerate() {
        left[idx+1] = left[idx].max(a[idx]);
        right[idx+1] = right[idx].max(a[n-idx-1]);
    }

    (0..n).for_each({|idx| {
        println!("{}", left[idx].max(right[n-idx-1]))
    }});

}
