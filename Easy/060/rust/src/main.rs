// https://atcoder.jp/contests/abc050/tasks/abc050_b
// B - Contest with Drinks Easy

// 問題文
// joisinoお姉ちゃんは、あるプログラミングコンテストの決勝を控えています。このコンテストでは、N 問の問題が用意されており、それらには 1～N の番号がついています。 joisinoお姉ちゃんは、問題 i(1≦i≦N) を解くのにかかる時間が Ti​ 秒であることを知っています。
// また、このコンテストでは、M 種類のドリンクが提供されており、1～M の番号がついています。そして、ドリンク i(1≦i≦M) を飲むと、脳が刺激され、問題 Pi​ を解くのにかかる時間が Xi​ 秒になります。他の問題を解くのにかかる時間に変化はありません。
// コンテスタントは、コンテスト開始前にいずれかのドリンクを 1 本だけ飲むことができます。 joisinoお姉ちゃんは、それぞれのドリンクについて、それを飲んだ際に、全ての問題を解くのに何秒必要なのかを知りたくなりました。全ての問題を解くのに必要な時間とは、それぞれの問題を解くのにかかる時間の合計です。
// あなたの仕事は、joisinoお姉ちゃんの代わりにこれを求めるプログラムを作成することです。

// 制約
//     入力は全て整数である
//     1≦N≦100
//     1≦Ti​≦105
//     1≦M≦100
//     1≦Pi​≦N
//     1≦Xi​≦105

// 入力
// 入力は以下の形式で標準入力から与えられる。
// N
// T1​ T2​ ... TN​
// M
// P1​ X1​
// P2​ X2​
// :
// PM​ XM​

// 出力
// それぞれのドリンクについて、それを飲んだ際に全ての問題を解くのに必要な時間を求め、順番に 1 行ずつ出力せよ。

use proconio::input;

fn content_with_drink(t: &Vec<u16>, px: &Vec<Vec<u16>>) -> Vec<u32> {
    let mut vec = Vec::with_capacity(px.len());

    let total: u32 = t.iter().map(|&x| x as u32).sum();
    for item in px.iter() {
        let problem_idx = (item[0] - 1) as usize;
        let time = item[1] as u32;

        vec.push((total + time - t[problem_idx] as u32) as u32);
    }
    vec
}

fn main() {
    input! {
        n: usize,
        t: [u16; n],
        m: usize,
        px: [[u16; 2] ; m],
    }

    let result = content_with_drink(&t, &px);
    for time in result.iter() {
        println!("{}", time);
    }
}

#[cfg(test)]
mod tests {
    use super::content_with_drink;
    #[test]
    fn samples() {
        assert_eq!(
            content_with_drink(&vec![2, 1, 4], &vec![vec![1, 1], vec![2, 3]]),
            vec![6, 9]
        );
        assert_eq!(
            content_with_drink(
                &vec![7, 2, 3, 8, 5],
                &vec![vec![04, 2], vec![1, 7], vec![4, 13]]
            ),
            vec![19, 25, 30]
        );
    }
}
