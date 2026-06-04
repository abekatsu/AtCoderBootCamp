// https://atcoder.jp/contests/abc139/tasks/abc139_c
// 左右一列に N 個のマスが並んでいます。
// 左から i 番目のマスの高さは Hi​ です。
// あなたは好きなマスに降り立ち、右隣のマスの高さが今居るマスの高さ以下である限り右隣のマスへ移動し続けます。
// 最大で何回移動できるでしょうか。

use proconio::input;

// 右へ連続して移動できる最大回数を返す。
// 右端から走査し、h[idx] >= h[idx+1] が続く限り連続移動数を伸ばす。
fn max_move_length(h: &[u64]) -> usize {
    let mut max_len = 0;

    let mut checked = vec![false; h.len()];
    for idx in 0..h.len() {
        if !checked[idx] {
            checked[idx] = true;
            let mut len = 0;
            for jdx in idx..h.len() - 1 {
                checked[jdx] = true;
                if h[jdx] < h[jdx + 1] {
                    break;
                }
                len += 1;
                if max_len < len {
                    max_len = len;
                }
            }
        }
    }

    // let mut max_len: usize = 0;
    // for i in 0..h.len() - 1 {
    //     let mut len = 0;
    //     for j in i..h.len() - 1 {
    //         if h[j] < h[j + 1] {
    //             break;
    //         }
    //         len += 1;
    //         if max_len < len {
    //             max_len = len;
    //         }
    //     }
    // }

    max_len
}

fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }

    println!("{}", max_move_length(&h));
}

#[cfg(test)]
mod tests {
    use super::max_move_length;

    #[test]
    fn samples() {
        assert_eq!(max_move_length(&[10, 4, 8, 7, 3]), 2);
        assert_eq!(max_move_length(&[4, 4, 5, 6, 6, 5, 5]), 3);
        assert_eq!(max_move_length(&[1, 2, 3, 4]), 0);
        assert_eq!(max_move_length(&[1, 5, 2, 4, 3]), 1);
        assert_eq!(max_move_length(&[4, 1, 2, 5, 3]), 1);
        assert_eq!(max_move_length(&[5, 4, 6]), 1);
        assert_eq!(max_move_length(&[4, 1, 2, 3, 5, 5, 5]), 2);
        assert_eq!(max_move_length(&[4, 1, 2, 6, 5, 5, 5]), 3);
        // 先頭まで続く連続移動。旧実装はここを取りこぼして 0 を返していた。
        assert_eq!(max_move_length(&[6, 5, 5, 5, 4, 1, 2]), 5);
        // 単調非増加。最長連が先頭から末尾まで続くケース（旧実装は 0 を返した）。
        assert_eq!(max_move_length(&[5, 4, 3, 2, 1]), 4);
    }
}
