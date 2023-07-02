use proconio::input;

fn main() {
    input! {
        n: u8,
        m: u8,
        c: i32,
        b: [i32; m],
        a: [[i32; m]; n]
    }

    let mut number_of_success_solve : u8= 0;

    for i in 0..(n as usize) {
        let mut score: i32 = 0;
        for j in 0..(m as usize) {
            score += a[i][j] * b[j];
        }
        score += c;
        if score > 0 { number_of_success_solve += 1 }
    }
    println!("{}", number_of_success_solve)
}
