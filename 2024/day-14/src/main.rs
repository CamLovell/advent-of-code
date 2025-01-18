use std::{collections::HashMap, fs, str::FromStr};

fn main() {
    let map = Map {
        width: 101,
        height: 103,
    };

    let mut robots: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| Robot::from_str(s).unwrap())
        .collect();

    map.show_all(&robots);
    for _ in 0..100 {
        robots.iter_mut().for_each(|r| {
            r.step(&map);
        });
        map.show_all(&robots);
    }
    // robots.iter_mut().for_each(|r| {
    //     r.run(100, &map);
    // });

    map.show_all(&robots);
    println!(
        "{:#?}",
        map.count_quadrants(&robots).iter().product::<i32>()
    )
}

#[derive(Debug)]
struct Robot {
    pos: Position,
    vel: Velocity,
}

impl Robot {
    fn step(&mut self, map: &Map) {
        self.pos.x = (self.pos.x + self.vel.x).rem_euclid(map.width);
        self.pos.y = (self.pos.y + self.vel.y).rem_euclid(map.height);
    }

    fn run(&mut self, steps: usize, map: &Map) {
        for _ in 0..steps {
            self.step(map);
        }
    }

    fn show_on_map(&self, map: &Map) {
        let mut display_str = String::new();
        for y in 0..map.height {
            for x in 0..map.width {
                display_str.push_str(if self.pos.x == x && self.pos.y == y {
                    "#"
                } else {
                    "."
                });
            }
            display_str.push_str("\n");
        }
        println!("{display_str}")
    }
}

impl FromStr for Robot {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(" ").ok_or("No split found on ' '")?;
        Ok(Robot {
            pos: Position::from_str(p)?,
            vel: Velocity::from_str(v)?,
        })
    }
}

#[derive(Debug)]
struct Map {
    height: i32,
    width: i32,
}

impl Map {
    fn show_all(&self, robots: &Vec<Robot>) {
        let mut locations: HashMap<Position, i32> = HashMap::new();
        robots.iter().for_each(|r| {
            locations
                .entry(r.pos.clone())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        });
        let mut display_str = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                match locations.get(&Position { x, y }) {
                    Some(c) => display_str.push_str(&c.to_string()),
                    None => display_str.push_str("."),
                };
            }
            display_str.push_str("\n");
        }
        println!("{display_str}")
    }

    fn count_quadrants(&self, robots: &Vec<Robot>) -> Vec<i32> {
        let horr = self.width / 2;
        let vert = self.height / 2;
        let mut quadrants: Vec<i32> = vec![0, 0, 0, 0];

        robots.iter().for_each(|r| {
            if r.pos.x < horr && r.pos.y < vert {
                quadrants[0] += 1
            } else if r.pos.x > horr && r.pos.y < vert {
                quadrants[1] += 1
            } else if r.pos.x < horr && r.pos.y > vert {
                quadrants[2] += 1
            } else if r.pos.x > horr && r.pos.y > vert {
                quadrants[3] += 1
            }
        });
        quadrants
    }
}

trait FromXYStr: Sized {
    /// Build type from x and y fields
    fn from_xy(x: i32, y: i32) -> Self;

    /// Parsing if default string `p=x,y`
    fn parse_str(s: &str) -> Result<Self, String> {
        let (x, y) = s
            .split_once("=")
            .ok_or("No split found on '='")?
            .1
            .split_once(",")
            .ok_or("No split found on ','")?;

        Ok(Self::from_xy(
            x.parse().map_err(|_| String::from("Could not parse x"))?,
            y.parse().map_err(|_| String::from("Could not parse y"))?,
        ))
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl FromXYStr for Position {
    fn from_xy(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl FromStr for Position {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Position::parse_str(s)
    }
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

impl FromXYStr for Velocity {
    fn from_xy(x: i32, y: i32) -> Self {
        Velocity { x, y }
    }
}

impl FromStr for Velocity {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Velocity::parse_str(s)
    }
}
