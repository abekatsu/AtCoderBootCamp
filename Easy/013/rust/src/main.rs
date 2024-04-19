use proconio::input;
use recursive::recursive;


// fn count_dividing(mut queue: &Vec::<[u32; 3]>) -> i32;

#[recursive]
fn count_dividing(queue: &mut Vec::<[u32; 3]>) -> i32 {
    let latest_value : [u32; 3] = *queue.last().unwrap_or_else(|| &[0, 0, 0]);
    let new_value: [u32; 3] = [ latest_value[1] / 2 + latest_value[2] / 2,  latest_value[2] / 2 + latest_value[0] / 2, latest_value[0] / 2 + latest_value[1] / 2 ];

    for num in latest_value {
        if num % 2 == 1 {
            return queue.len() as i32 - 1;
        }
    }

    for value in queue.into_iter() {
        let matching = new_value.iter().zip(value).filter(|(a, b)| a == b).count();

        if matching == 3 {
            return -1;
        }
    }

    queue.push(new_value);
    count_dividing(queue)
}


fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32
    }

    let mut queue: Vec::<[u32; 3]> = Vec::new();
    queue.push([a, b, c]);
    println!("{}", count_dividing(&mut queue));
}
