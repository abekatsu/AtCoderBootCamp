// https://atcoder.jp/contests/abc083/tasks/abc083_b
// 1 以上 N 以下の整数のうち、10 進法での各桁の和が A 以上 B 以下であるものの総和を求めてください。
// 制約
//    1≤N≤10^4
//    1≤A≤B≤36
//    入力はすべて整数である
// 入力
// 入力は以下の形式で標準入力から与えられる。
// N A B
// 出力
// 1 以上 N 以下の整数のうち、10 進法での各桁の和が A 以上 B 以下であるものの総和を出力せよ。

use proconio::input;

fn digit_sum(n: usize) -> usize {
    let mut num = n;
    let mut sum: usize = 0;

    loop {
        sum += num % 10;
        num /= 10;
        if num == 0 {
            break;
        }
    }

    sum
}

fn some_sums(n: usize, a: u8, b: u8) -> usize {
    let mut sums = 0;
    for num in 1..=n {
        let digit_sum = digit_sum(num);
        if a as usize <= digit_sum && digit_sum <= b as usize {
            sums += num;
        }
    }

    sums
}

fn main() {
    input! {
        n: usize,
        a: u8,
        b: u8,
    }

    println!("{}", some_sums(n, a, b));
}

#[cfg(test)]
mod tests {
    use super::some_sums;

    #[test]
    fn samples() {
        assert_eq!(some_sums(20, 2, 5), 84);
        assert_eq!(some_sums(10, 1, 2), 13);
        assert_eq!(some_sums(100, 4, 16), 4554);
    }
}
