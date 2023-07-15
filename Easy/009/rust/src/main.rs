use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n]
    }

    let mut result_norm: usize = 0;
    for i in 0..n {
        // ロボットA の移動距離を求める
        let robot_a_move_norm : usize = x[i];
        let robot_b_move_norm : usize = k - x[i];
        if robot_a_move_norm < robot_b_move_norm {
            result_norm += (robot_a_move_norm) * 2;
        } else {
            result_norm += (robot_b_move_norm) * 2;
        }
    }

    println!("{}", result_norm);

    println!("{}",  x.iter().map(|pos| {
        return if *pos < k - *pos { (*pos) * 2 } else { (k - *pos)*2 };
    }).collect::<Vec<usize>>().iter().sum::<usize>());
}
