use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (rule_list, pages_str) = input.split_once("\n\n").unwrap();

    let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for rule_str in rule_list.lines() {
        let (lhs, rhs) = rule_str.split_once("|").unwrap();
        rule_map
            .entry(rhs.parse().unwrap())
            .and_modify(|firsts| firsts.push(lhs.parse().unwrap()))
            .or_insert(vec![lhs.parse().unwrap()]);
    }

    let mut updates: Vec<Vec<usize>> = pages_str
        .lines()
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    let part_1: usize = updates
        .iter()
        .filter(|update| is_ordered(&update, &rule_map))
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{part_1:?}");

    let part_2: usize = updates
        .iter_mut()
        .filter(|update| !is_ordered(&update, &rule_map))
        .map(|update| sort_update(update, &rule_map))
        .map(|update| update[update.len() / 2])
        .sum();
    println!("{part_2:?}");
}

fn sort_update<'a>(update: &'a mut [usize], rule_map: &HashMap<usize, Vec<usize>>) -> &'a [usize] {
    let mut pos = 0;
    while pos < update.len() {
        let mut swap_pos: Option<usize> = None;
        if let Some(rule) = rule_map.get(&update[pos]) {
            for i in pos + 1..update.len() {
                if rule.contains(&update[i]) {
                    swap_pos = Some(i)
                }
            }
        }
        match swap_pos {
            Some(idx) => {
                update.swap(pos, idx);
            }
            None => pos += 1,
        }
    }
    update
}

fn is_ordered(update: &[usize], rule_map: &HashMap<usize, Vec<usize>>) -> bool {
    let mut bad: HashSet<usize> = HashSet::new();
    for page in update {
        if bad.contains(page) {
            return false;
        }
        if let Some(rule) = rule_map.get(page) {
            rule.iter().for_each(|elem| {
                bad.insert(*elem);
            });
        }
    }
    true
}
