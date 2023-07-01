use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    let mut number_of_sockets: i32 = 1;
    let mut number_of_taps: i32 = 0;

    while number_of_sockets < b {
        number_of_sockets += a - 1;
        number_of_taps += 1;
    }

    println!("{}", number_of_taps);
}
