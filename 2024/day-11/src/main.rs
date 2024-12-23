use rayon::prelude::*;
use std::{collections::HashMap, fs};
fn main() {
    let stones: Vec<u64> = fs::read_to_string("input.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Gotta be a better way than a dirty clone
    let total: u64 = stones
        .par_iter()
        .map(|&stone| blink_stone(stone, 0, 75, &mut HashMap::new()))
        .sum();
    println!("{:?}", total);
}

fn blink_stone(stone: u64, iter: usize, max: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
    if let Some(res) = cache.get(&(stone, iter)) {
        return *res;
    }

    let result: u64;
    if iter >= max {
        result = 1;
    } else {
        if stone == 0 {
            result = blink_stone(1, iter + 1, max, cache);
        } else {
            let num_digits = stone.to_string().chars().count() as u32;
            if num_digits % 2 == 0 {
                let offset = (10 as u64).pow(num_digits / 2);
                let left = stone / offset; // u64 so get floor for free
                let right = stone - left * offset;

                result = blink_stone(left, iter + 1, max, cache)
                    + blink_stone(right, iter + 1, max, cache);
            } else {
                result = blink_stone(stone * 2024, iter + 1, max, cache);
            }
        }
    }

    cache.insert((stone, iter), result);
    return result;
}
