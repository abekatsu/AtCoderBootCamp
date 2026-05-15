use proconio::input;

// あなたは整数 x を持っています。最初、x=0 です。
// あなたは、長さ N の文字列 S をもらったので、これを使って N 回の操作を行いました。
// i 回目の操作では、Si​=I ならば x の値を 1 増やし、Si​=D ならば x の値を 1 減らしました。
// 操作の途中( 1 回目の操作の前、N 回目の操作の後も含む)で x がとる値の最大値を答えてください。

fn main() {
    input! {
        _n: u32,
        s: String,
    }
    let mut x: i32  = 0;
    let mut max_x: i32 = x;

    s.chars().for_each(|c| {
        match c {
            'D' => { x-=1; },
            'I' => { x+=1; if max_x < x { max_x = x } ;} ,
            _ => {},
        }
    });
    println!("{}", max_x);
}
