use proconio::input;
fn main() {
    input! {
        mut height: u64,
        mut width: u64
    }

    if height < width {
        let tmp: u64 = width;
        width = height;
        height = tmp;
    }
    
    let result : u64 = if height == 1 || width == 1 { 1 }
    else if height % 2 != 0 && width % 2 != 0 { height * width / 2 + 1}
    else if height % 2 != 0 { width / 2 * height }
    else if width % 2 != 0 { height / 2 * width }
    else { height / 2 * width };
    
    println!("{}", result);
}
