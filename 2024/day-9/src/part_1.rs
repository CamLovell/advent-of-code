use std::fs;

pub fn run() {
    let disk_map: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .filter_map(|char| char.to_digit(10))
        .collect();

    let mut id = 0;
    let mut expanded_map: Vec<i32> = Vec::new();
    for (i, num) in disk_map.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*num {
                expanded_map.push(id); // fix this
            }
            id += 1
        } else {
            for _ in 0..*num {
                expanded_map.push(-1);
            }
        }
    }

    let mut free_ptr = 0;
    let mut end_ptr = expanded_map.len() - 1;

    println!("{:?}", expanded_map);
    loop {
        free_ptr = match step_free_ptr(free_ptr, &expanded_map) {
            Some(ptr) => ptr,
            None => break,
        };
        end_ptr = match step_end_ptr(end_ptr, &expanded_map) {
            Some(ptr) => ptr,
            None => break,
        };
        if free_ptr >= end_ptr {
            break;
        }

        let tmp = expanded_map[free_ptr];
        expanded_map[free_ptr] = expanded_map[end_ptr];
        expanded_map[end_ptr] = tmp;
    }
    let check_sum: usize = expanded_map
        .iter()
        .filter(|file| **file >= 0)
        .enumerate()
        .map(|(i, num)| i * (*num as usize))
        .sum();

    println!("{}", check_sum);
}

fn step_free_ptr(mut ptr: usize, map: &Vec<i32>) -> Option<usize> {
    loop {
        if ptr >= map.len() {
            return None;
        }

        // println!("{}", map[ptr]);
        match map[ptr] {
            -1 => return Some(ptr),
            _ => {
                ptr += 1;
                continue;
            }
        }
    }
}

fn step_end_ptr(mut ptr: usize, map: &Vec<i32>) -> Option<usize> {
    loop {
        if ptr == 0 {
            return None;
        }
        match map[ptr] {
            -1 => {
                ptr -= 1;
                continue;
            }
            _ => return Some(ptr),
        }
    }
}
