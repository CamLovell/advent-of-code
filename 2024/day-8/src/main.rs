use std::{collections::HashSet, fs, str::FromStr};

fn main() {
    let map = Map::from_str(&fs::read_to_string("input.txt").unwrap()).unwrap();

    let antinodes = map.get_antinodes().unwrap();
    let mut map_display = String::new();
    for y in 0..map.height {
        for x in 0..map.width {
            if let Some(c) = map.antenna_at(&x, &y) {
                map_display.push(c);
            } else if antinodes.contains(&Antinode { x, y }) {
                map_display.push('#');
            } else {
                map_display.push('.');
            }
        }
        map_display.push_str("\n")
    }

    println!("{map_display}");
    println!("{}", antinodes.len());
}

#[derive(Debug)]
struct Map {
    width: i32,
    height: i32,
    antennas: Vec<Antenna>,
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas: Vec<Antenna> = Vec::new();

        let lines: Vec<_> = s.lines().collect();
        let width: i32 = lines
            .len()
            .try_into()
            .map_err(|_| String::from("Error getting width"))?;
        let height: i32 = lines[0]
            .chars()
            .collect::<Vec<_>>()
            .len()
            .try_into()
            .map_err(|_| String::from("Error getting width"))?;

        for (y, line) in lines.iter().enumerate() {
            for (x, freq) in line.chars().enumerate() {
                if freq != '.' {
                    antennas.push(Antenna {
                        x: x.try_into().map_err(|_| "int parse failed")?,
                        y: y.try_into().map_err(|_| "int parse failed")?,
                        freq,
                    });
                }
            }
        }

        Ok(Map {
            width,
            height,
            antennas,
        })
    }
}

impl Map {
    fn get_antinodes(&self) -> Option<HashSet<Antinode>> {
        let mut antinodes: HashSet<Antinode> = HashSet::new();
        for (i, antenna) in self.antennas.iter().enumerate() {
            for other in &self.antennas[i + 1..] {
                let mut scale = 0;
                loop {
                    if let Some((node_1, node_2)) = antenna.get_antinodes(other, &scale) {
                        let has_node_1 = self.contains(&node_1);
                        let has_node_2 = self.contains(&node_2);
                        if has_node_1 {
                            antinodes.insert(node_1);
                        }
                        if has_node_2 {
                            antinodes.insert(node_2);
                        }
                        scale += 1;
                        if !has_node_1 && !has_node_2 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        match antinodes.len() {
            0 => None,
            _ => Some(antinodes),
        }
    }

    fn contains(&self, node: &Antinode) -> bool {
        node.x < self.width && node.y < self.height && node.x >= 0 && node.y >= 0
    }

    fn antenna_at(&self, x: &i32, y: &i32) -> Option<char> {
        for antenna in self.antennas.iter() {
            if antenna.x == *x && antenna.y == *y {
                return Some(antenna.freq);
            }
        }
        None
    }
}
// Node can be an antenna or an antinode
#[derive(Debug)]
struct Antenna {
    x: i32,
    y: i32,
    freq: char,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Antinode {
    x: i32,
    y: i32,
}

impl Antenna {
    fn get_antinodes(&self, other: &Self, scale: &i32) -> Option<(Antinode, Antinode)> {
        if self.freq != other.freq {
            return None;
        }

        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;

        Some((
            Antinode {
                x: self.x + scale * x_diff,
                y: self.y + scale * y_diff,
            },
            Antinode {
                x: self.x - scale * x_diff,
                y: self.y - scale * y_diff,
            },
        ))
    }
}
