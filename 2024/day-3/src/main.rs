use std::fs;

use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't|do").unwrap();
    let data = fs::read_to_string("input.txt").expect("Read failed");
    let mut sum = 0;
    let mut enabled = true;
    re.captures_iter(&data).for_each(|item| match item.get(0) {
        Some(s) if s.as_str() == "do" => enabled = true,
        Some(s) if s.as_str() == "don't" => enabled = false,
        Some(_) if enabled => {
            sum += item.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * item.get(2).unwrap().as_str().parse::<i32>().unwrap()
        }
        Some(_) => {} // Skipped case
        None => panic!("No clue how this could even happen"),
    });
    // let mut sum = 0;
    // for (_, [lhs, rhs]) in re.captures_iter(&data).map(|c| c.extract()) {
    //     sum += lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap()
    // }
    //
    println!("{sum}");
}
