use proconio::input;
use std::collections::VecDeque;

fn generate_prime_list(max_size: u32) -> Vec<u32> {
    let p: u32 = ((max_size + 1) as f32).sqrt().ceil() as u32;
    let capacity: usize = p.pow(2u32) as usize;

    let mut array = VecDeque::<u32>::with_capacity(capacity);
    let mut primes = Vec::<u32>::new();
    for i in 2..capacity+1 { array.push_back(i as u32); }

    loop {
        let value = array.pop_front();
        match value.as_ref() {
            Some(prime) => {
                primes.push(*prime);
                array = array.iter().filter(|&&v| v % *prime != 0).cloned().collect::<VecDeque<_>>();
                if *prime > p {
                    array.iter().for_each(|&v| primes.push(v));
                    break;
                }
            },
            None => { break; }
        }
    }

    return primes;
}

fn main() {
    input! {
        x: u32
    }
    let primes = generate_prime_list(x);
    if primes.len() > 0 {
        for &prime in primes.iter() {
            if prime >= x {
                println!("{}", prime);
                break;
            }
        }
    }
}
