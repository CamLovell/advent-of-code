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
        .map(|&stone| blink_stone(stone, 0, 42, &mut HashMap::new()))
        .sum();
    println!("{:?}", total);
}

fn blink_stone(stone: u64, iter: usize, max: usize, cache: &mut HashMap<u64, Vec<u64>>) -> u64 {
    if let Some(v) = cache.get(&stone) {
        if v.len() > iter {
            return v[iter];
        }
    }

    if iter >= max {
        cache.entry(stone).and_modify(|v| v.push(1));
        return 1;
    }

    if stone == 0 {
        let result = blink_stone(1, iter + 1, max, cache);
        cache.entry(stone).and_modify(|v| v.insert(iter, result));
        return result;
    }
    let num_digits = stone.to_string().chars().count() as u32;
    if num_digits % 2 == 0 {
        let offset = (10 as u64).pow(num_digits / 2);
        let left = stone / offset; // u64 so get floor for free
        let right = stone - left * offset;

        return blink_stone(left, iter + 1, max, cache) + blink_stone(right, iter + 1, max, cache);
    } else {
        return blink_stone(stone * 2024, iter + 1, max, cache);
    }
}
