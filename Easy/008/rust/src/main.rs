use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32
    }
    let mut number: u32 = 0;
    if b <= 9 {
        number = a * 10 + b
    } else if b < 100 {
        number = a * 100 + b
    } else if b == 100 {
        number = a * 1000 + b
    }

    let mut test_number: u32 = 1;
    let mut result: bool = false;
    while test_number.pow(2) <= number {
        // println!("{}, `{}`", test_number.pow(2), number);
        if test_number.pow(2) == number {
            result = true
        }
        test_number += 1;
    }
    println!("{}", if result { "Yes" } else {"No"});
}
