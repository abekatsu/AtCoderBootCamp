use proconio::input;
fn main() {
    input! {
        n: usize,
        mut d: [i32; n]
    }

    d.sort();
    let idx: usize = n / 2;
    println!("{}", d[idx] - d[idx - 1]);
}
