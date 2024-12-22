use std::{char, fs, num::ParseIntError, str::FromStr};

fn main() {
    // Extract data into vec of stucts
    let test_cases: i64 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| TestCase::from_str(line).expect(&format!("{line}")))
        .filter(|elem| elem.run_test().is_some())
        .map(|elem| elem.ans)
        .sum();

    println!("{:?}", test_cases)
}

fn int_concat(lhs: i64, rhs: i64) -> Result<i64, ParseIntError> {
    format!("{lhs}{rhs}").parse()
}

#[derive(Debug)]
struct TestCase {
    ans: i64,
    numbers: Vec<i64>,
}

impl TestCase {
    fn run_test(&self) -> Option<Vec<char>> {
        if self.numbers.len() < 2 {
            return None;
        }

        Self::check_vec(&self.numbers, &self.ans)
    }

    fn check_vec(vec: &[i64], ans: &i64) -> Option<Vec<char>> {
        let start = &vec[0..=1];
        let mut rest = vec[2..].to_vec();

        let sum: i64 = start.iter().sum();
        let prod: i64 = start.iter().product();
        let concat: i64 = int_concat(start[0], start[1]).unwrap();

        if rest.len() == 0 {
            if sum == *ans {
                return Some(vec!['+']);
            } else if prod == *ans {
                return Some(vec!['*']);
            } else if concat == *ans {
                return Some(vec!['|']);
            } else {
                return None;
            }
        }

        rest.insert(0, sum);
        if let Some(mut res) = Self::check_vec(&rest, ans) {
            res.insert(0, '+');
            return Some(res);
        }

        rest[0] = prod;
        if let Some(mut res) = Self::check_vec(&rest, ans) {
            res.insert(0, '*');
            return Some(res);
        }

        rest[0] = concat;
        if let Some(mut res) = Self::check_vec(&rest, ans) {
            res.insert(0, '|');
            return Some(res);
        }

        None
    }
}

impl FromStr for TestCase {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ans, numbers) = match s.split_once(":") {
            Some(split) => (
                split
                    .0
                    .parse()
                    .map_err(|_| format!("error parsing ans {split:?}"))?,
                split
                    .1
                    .split_whitespace()
                    .map(|num| {
                        num.parse::<i64>()
                            .map_err(|_| String::from("eror parsing value"))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            None => return Err(String::from("Error splitting string")),
        };

        Ok(TestCase { ans, numbers })
    }
}
