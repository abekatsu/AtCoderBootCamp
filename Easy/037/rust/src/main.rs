use proconio::input;

// https://atcoder.jp/contests/abc141/tasks/abc141_c
// 高橋君は早押しクイズの大会を開くことにしました。スコアボードの作成を任されたキザハシ君は、次のルールを持つラウンドのポイントを管理するプログラムを書くのに苦戦しています。
// このラウンドの参加者は N 人であり、1 から N までの番号がついています。ラウンド開始時点では全員が K ポイントを持っています。
// 誰かが問題に正解すると、その人以外の N−1 人のポイントが 1 減ります。これ以外によるポイントの変動はありません。
// ラウンド終了時にポイントが 0 以下の参加者は敗退し、残りの参加者が勝ち抜けます。
// このラウンドでは Q 回の正解が出て、i 番目に正解したのは参加者 Ai​ でした。キザハシ君の代わりに、N 人の参加者のそれぞれが勝ち抜けたか敗退したかを求めるプログラムを作成してください。

fn main() {
    input! {
        n: usize,
        k: i64,
        q: usize,
        a: [usize; q],
    }

    // 回答者の正解数
    let mut correct = vec![0i64; n + 1];
    a.iter().for_each(|&ans| correct[ans] +=1) ;


    // 回答者ごとに最終ポイントを算出する。( K - Q + correct[n] )
    (1..=n).for_each(|i| {
        let final_point = k - q as i64 + correct[i];
        if final_point > 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    });

}
