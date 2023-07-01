use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i32; n],
    }
    let min = x.iter().min().unwrap();
    let max = x.iter().max().unwrap();

    let mut min_power = 0;
    let mut i: i32 = *min;
    while i <= *max {
        let mut j: usize = 0;
        let mut power: u32 = 0;
        while j < n {
            power += (x[j] - i).pow(2) as u32;
            j += 1;
        }
        if min_power == 0 {
            min_power = power;
        } else if power < min_power {
            min_power = power;
        } else {
            break;
        }
        i += 1;
    }
    println!("{}", min_power);
}
