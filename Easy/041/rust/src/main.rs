// https://atcoder.jp/contests/abc152/tasks/abc152_c
// 1,…,N の順列 P1​,…,PN​ が与えられます。
// 次の条件を満たす整数 i(1≤i≤N) の個数を数えてください。
//   任意の整数 j(1≤j≤i) に対して、 Pi​≤Pj​

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32; n],
    }

    let (_, result) = (1..n).fold((p[0], 1u32), |(least, count), idx| {
        if least > p[idx] {
            (p[idx], count + 1)
        } else {
            (least, count)
        }
    });

    println!("{}", result);
}
