use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let price_excluding_tax: u64 = (n as f64 / 1.08).floor() as u64;

    let mut count = 0;
    loop {
        if ((price_excluding_tax + count) * 108) / 100 == n {
            println!("{}", price_excluding_tax + count);
            break;
        } else if ((price_excluding_tax + count) * 108) / 100 > n {
            println!(":(");
            break;
        }
        count += 1;
    }
}
