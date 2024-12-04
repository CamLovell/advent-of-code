use std::{char, fs};

fn main() {
    let mut word_search: Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        word_search.push(line.chars().collect());
    }

    let num_rows = word_search.len();
    let num_cols = word_search[0].len();

    let mut count = 0;

    // Move through word search grid and check everything
    for row in 0..num_rows {
        for col in 0..num_cols {
            // check forward if col <= len -4
            if col <= num_cols - 4 {
                let word: String = word_search
                    .get(row)
                    .unwrap()
                    .get(col..col + 4)
                    .unwrap()
                    .iter()
                    .collect();

                if check_word(&word) {
                    count += 1;
                }
            }
            // check downward if row <= len-4
            if row <= num_rows - 4 {
                let word: String = word_search
                    .get(row..row + 4)
                    .unwrap()
                    .iter()
                    .map(|column| column.get(col).unwrap())
                    .collect();

                if check_word(&word) {
                    count += 1;
                }
            }
            // check forward diagonal if row and col < len-4
            if row <= num_rows - 4 && col <= num_cols - 4 {
                let word: String = word_search
                    .get(row..row + 4)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(i, column)| column.get(col + i).unwrap())
                    .collect();

                if check_word(&word) {
                    count += 1;
                }
            }
            // check backward diagonal if col >= 3 and row < len-4
            if row <= num_rows - 4 && col >= 3 {
                let word: String = word_search
                    .get(row..row + 4)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(i, column)| column.get(col - i).unwrap())
                    .collect();

                if check_word(&word) {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");
}

fn check_word(word: &str) -> bool {
    word == "XMAS" || word == "SAMX"
}
