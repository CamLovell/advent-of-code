use core::panic;
use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{args:?}");
    if args.len() != 3 {
        panic!("Please provide part and path");
    }

    let part = &args[1];
    let path = &args[2];

    let (mut left, mut right) = load_columns(path);

    let result = match part.parse::<u32>() {
        Ok(1) => part_1(&mut left, &mut right),
        Ok(2) => part_2(&mut left, &mut right),
        Ok(n) => panic!("Invalid part {n}"),
        Err(_) => panic!("Could not parse part as u32"),
    };
    println!("{result}")
}

fn part_2(left: &mut Vec<i64>, right: &mut Vec<i64>) -> i64 {
    let mut left_map: HashMap<_, _> = left.iter().map(|val| (val, 0)).collect();

    for val in right {
        left_map.entry(val).and_modify(|count| *count += 1);
    }
    left.iter()
        .map(|key| key * left_map.get(key).unwrap())
        .sum()
}

fn part_1(left: &mut Vec<i64>, right: &mut Vec<i64>) -> i64 {
    // Sort lhs and rhs
    left.sort();
    right.sort();

    // Get sum
    left.iter().zip(right).map(|(a, b)| (*a - *b).abs()).sum()
}

fn load_columns(path: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    // Load values from file into left and right
    for line in fs::read_to_string(path).unwrap().lines() {
        let vals: Vec<_> = line
            .split_whitespace()
            .map(|val| val.parse::<i64>().unwrap())
            .collect();

        left.push(vals[0]);
        right.push(vals[1]);
    }

    (left, right)
}
