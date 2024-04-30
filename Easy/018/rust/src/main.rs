use proconio::input;
use std::cmp;

fn main() {
    input! {
        str: String
    }

    let mut max_length = 0;
    let mut current_length = 0;

    str.chars().for_each(|c| {
        match c {
            'A' | 'C' | 'G' | 'T' => current_length += 1,
            _ => {
                current_length = 0;
            }
        }
        max_length = cmp::max(max_length, current_length);
    });
    println!("{}", max_length);
}
