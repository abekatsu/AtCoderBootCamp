use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: String,
    }
    println!("Hello, world!");
    println!("n {}, a: {}, b: {}, s: {}", n, a, b, s);

    for (idx, ch) in s.chars().enumerate() {
	println!("{} - {}", idx, ch)
    }
}
