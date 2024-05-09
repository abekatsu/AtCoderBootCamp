use proconio::input;
use std::cmp;

fn main() {
    input! {
        _n: usize,
        m: usize,
        x: usize,
        a: [usize; m]
    }

    // let process_1 = a.iter().filter(|&v| *v >= 1 && *v <= x  - 1).collect::<Vec<_>>().len();
    // let process_2 = a.iter().filter(|&v| *v >= x + 1 && *v <= n - 1).collect::<Vec<_>>().len();

    let mut process_1 : usize = 0;
    let mut process_2 : usize = 0;
    a.iter().for_each(|v|
        if *v < x {
            process_1 += 1;
        } else {
            process_2 += 1;
        }
    );

    println!("{}", cmp::min(process_1, process_2));

}
