use proconio::input;
use std::collections::LinkedList;

fn main() {
    input! {
        s: u32,
    }

    let mut list:LinkedList<u32> = LinkedList::new();
    list.push_back(s);

    loop {
        let prev_s = *list.back().unwrap();
        let next_s: u32;
        if prev_s % 2 == 0 {
            next_s = prev_s / 2;
        } else {
            next_s = prev_s * 3 + 1;
        }

        let iter = list.iter().position(|&s| s == next_s);
        if iter != None {
            println!("{}", list.len() + 1);
            break;
        } else {
            list.push_back(next_s);
        }
    }
}
