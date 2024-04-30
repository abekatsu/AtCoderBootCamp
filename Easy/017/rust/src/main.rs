use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [f32; n]
    }

    let mut stack = Vec::<f32>::new();
    v.iter().for_each(|_v| stack.push( *_v as f32));
    stack.sort_by(|l, r| r.partial_cmp(l).unwrap() ); // sort by reverse order.

    while stack.len() > 1 {
        let first = stack.pop().unwrap();
        let second = stack.pop().unwrap();
        stack.push( (first + second) / 2.0 );
    }

    println!("{}", stack.pop().unwrap());
}
