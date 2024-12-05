use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (rule_list, pages_str) = input.split_once("\n\n").unwrap();

    let mut rule_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for rule_str in rule_list.lines() {
        let (lhs, rhs) = rule_str.split_once("|").unwrap();
        rule_map
            .entry(rhs.parse().unwrap())
            .and_modify(|firsts| firsts.push(lhs.parse().unwrap()))
            .or_insert(vec![lhs.parse().unwrap()]);
    }

    let updates: Vec<Vec<u32>> = pages_str
        .lines()
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    for up in updates.iter() {
        if up.len() % 2 == 0 {
            println!("NO MIDDLE {:?}", up);
            panic!("");
        }
    }

    let part_1: u32 = updates
        .iter()
        .filter(|update| is_ordered(&update, &rule_map))
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{part_1:?}")
}

fn is_ordered(update: &[u32], rule_map: &HashMap<u32, Vec<u32>>) -> bool {
    let mut bad: HashSet<u32> = HashSet::new();
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
