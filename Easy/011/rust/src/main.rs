use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut result: u32 = 1;

    loop {
        if result > n { break; }
        result = result << 1;
    }
    println!("{}", result >> 1);
}
