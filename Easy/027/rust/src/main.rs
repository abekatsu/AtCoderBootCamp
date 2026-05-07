use proconio::input;

fn number_of_bull_balls(n: usize, a: usize, b:usize) -> usize {
    if a == 0 { return 0; }

    let cycle = a + b;
    let full = n / cycle;
    let rem = n % cycle;

    return full * a + rem.min(a)
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    println!("{}", number_of_bull_balls(n, a, b));
}
