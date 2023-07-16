use proconio::input;

fn main() {
    input! {
        k: u32,
        n: usize,
        a: [u32; n]
    }

    let mut distance = Vec::<u32>::new();

    for index in 0..n-1 {
        distance.push(a[index + 1] - a[index])
    }
    distance.push(k - a[n-1] + a[0]);
    let max_distance: u32 = *distance.iter().max().unwrap();
    println!("{}", distance.iter().sum::<u32>() - max_distance);
}
