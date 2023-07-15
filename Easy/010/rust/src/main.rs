use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    }
    a.sort_by(|aa, bb| bb.cmp(aa));
    println!("{:?}", a.iter().enumerate().map( |(index, &val)| {
        return if index % 2 == 0 { val } else { - val };
    }).collect::<Vec<i32>>().iter().sum::<i32>());
}
