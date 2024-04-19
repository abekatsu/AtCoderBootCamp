use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64
    }

    // n = pk + q where p is a non-negative integer and 0<= q< k
    let p = n / k;
    let q = n - p * k;
    // println!("{} = {} * {} + {}", n, p, k, q);
    // println!("k/2: {}, q: {}", k/2, q);

    let ret: u64;
    if k < 2*q {
        ret = k - q;
    } else {
        ret = q;
    }
    println!("{}", ret);
}
