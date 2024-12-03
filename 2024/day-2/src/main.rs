use std::fs;
fn main() {
    let reports = better_extract_reports("input.txt");

    let result = reports.iter().filter(|report| is_safe(report)).count();
    println!("{result:?}")
}

fn is_safe(report: &Vec<i32>) -> bool {
    for i in 0..report.len() - 1 {
        let current = report[i];
        let next = report[i + 1];
        let diff = next - current;

        if diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }

        // This kinda sucks, want to fix it!
        let direction = get_direction(&report[1], &report[0]);
        if (diff / diff.abs()) != direction {
            return false;
        }
    }
    true
}

fn get_direction(lhs: &i32, rhs: &i32) -> i32 {
    (lhs - rhs) / (lhs - rhs).abs()
}

fn better_extract_reports(path: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}
