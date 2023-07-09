use proconio::input;

fn main() {
    input! {
        a: [[u32; 3]; 3],
        n: usize,
        b: [u32; n]
    }

    let mut result:bool = false;
    let mut c: [[u32; 3]; 3] = [[0; 3]; 3];
    for num_b in b {
        for i in 0..3 {
            for j in 0..3 {
                if num_b == a[i][j] { c[i][j] = 1; }
            }
        }
    };

    // check a[i][0], a[i][1], a[i][2],
    for i in 0..3 {
        if c[i][0] == 1 && c[i][1] == 1 && c[i][2] == 1 {
            result = true;
        }
    }

    // check a[0][j], a[1][j], a[2][j],
    for j in 0..3 {
        if c[0][j] == 1 && c[1][j] == 1 && c[2][j] == 1 {
            result = true;
        }
    }

    // check a[0][0], a[1][1], a[2][2]
    // check a[2][0], a[1][1], a[0][2]
    if c[0][0] == 1 && c[1][1] == 1 && c[2][2] == 1 {
        result = true;
    }

    if c[2][0] == 1 && c[1][1] == 1 && c[0][2] == 1 {
        result = true;
    }

    println!("{}", if result { "Yes" } else { "No" });
}
