use std::{collections::HashMap, fmt::Display, fs, str::FromStr};

fn main() {
    let map = Map::from_str(&fs::read_to_string("input.txt").unwrap()).unwrap();

    let mut trailheads: Vec<Location> = Vec::new();
    for (row, row_data) in map.map.iter().enumerate() {
        for (col, elem) in row_data.iter().enumerate() {
            if *elem == 0 {
                trailheads.push(Location {
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }

    let mut part_1 = 0;
    let mut part_2 = 0;
    for head in trailheads {
        let mut endpoints: HashMap<Location, i32> = HashMap::new();
        get_endpoints(&head, &map, &mut endpoints);
        part_1 += endpoints.len();
        part_2 += endpoints.iter().map(|(_, v)| v).sum::<i32>();
    }
    println!("{part_1}");
    println!("{part_2}");
}

fn check_step(next: &Location, current: &i32, map: &Map, endpoints: &mut HashMap<Location, i32>) {
    if map.contains(next) && (map.val_at(next) - current) == 1 {
        if map.val_at(next) == 9 {
            endpoints
                .entry(next.clone())
                .and_modify(|i| {
                    *i += 1;
                })
                .or_insert(1);
        } else {
            get_endpoints(&next, map, endpoints);
        }
    }
}
fn get_endpoints(loc: &Location, map: &Map, endpoints: &mut HashMap<Location, i32>) {
    let current = map.val_at(loc);
    // Up
    let next = Location {
        row: loc.row - 1,
        col: loc.col,
    };
    check_step(&next, &current, map, endpoints);
    // Down
    let next = Location {
        row: loc.row + 1,
        col: loc.col,
    };
    check_step(&next, &current, map, endpoints);
    // Left
    let next = Location {
        row: loc.row,
        col: loc.col - 1,
    };
    check_step(&next, &current, map, endpoints);
    // Right
    let next = Location {
        row: loc.row,
        col: loc.col + 1,
    };
    check_step(&next, &current, map, endpoints);
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<i32>>,
    height: i32,
    width: i32,
}

impl Map {
    fn val_at(&self, loc: &Location) -> i32 {
        self.map[loc.row as usize][loc.col as usize]
    }
    fn contains(&self, loc: &Location) -> bool {
        loc.row >= 0 && loc.row < self.height && loc.col >= 0 && loc.col < self.width
    }
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<i32>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect();
        let height = map.len() as i32;
        let width = map[0].len() as i32;

        Ok(Map { map, height, width })
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in &self.map {
            for col in row {
                output.push_str(&col.to_string());
            }
            output.push_str("\n");
        }
        write!(f, "{output}")
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Location {
    row: i32,
    col: i32,
}
