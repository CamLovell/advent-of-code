use std::fs;
fn main() {
    let reports = extract_reports("input.txt");

    let result = reports.iter().filter(|report| is_safe(report)).count();
    println!("Part 1: {result:?}");

    let result = reports.iter().filter(|report| is_kina_safe(report)).count();
    println!("Part 2: {result:?}");
}
fn is_kina_safe(report: &[i32]) -> bool {
    is_safe(report)
        || (0..report.len()).any(|i| {
            let mut working = report.to_vec();
            working.remove(i);
            is_safe(&working)
        })
}

fn is_safe(report: &[i32]) -> bool {
    let direction = (report[0] - report[1]).signum();
    report.windows(2).all(|pair| {
        (1..=3).contains(&(pair[0] - pair[1]).abs()) && (pair[0] - pair[1]).signum() == direction
    })
}

fn extract_reports(path: &str) -> Vec<Vec<i32>> {
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
