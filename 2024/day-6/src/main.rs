use std::{collections::HashSet, fmt, fs, str::FromStr, thread, time::Duration};
fn main() {
    let map_str = fs::read_to_string("input.txt").unwrap();
    let mut map = Map::from_str(&map_str).unwrap();

    // println!("{:#?}", map);
    // This feels weird??
    // while map.step(true) {}

    loop {
        if false {
            println!("{map}");
            thread::sleep(Duration::from_millis(500));
        }
        if !map.step(true) {
            break;
        }
    }

    let mut cells = HashSet::new();
    map.visited.iter().for_each(|guard| {
        cells.insert(guard.loc.clone());
    });
    println!("{}", cells.len());
    println!("{}", map.visited.len());
    // println!("{}", map);
    println!("{}", map.new_locs.len());
}

#[derive(Debug)]
struct Map {
    obstacles: HashSet<Position>,
    visited: HashSet<Guard>,
    new_locs: HashSet<Position>,
    height: usize,
    width: usize,
    guard: Guard,
}
impl Map {
    fn step(&mut self, check_loops: bool) -> bool {
        // Point the guard is facing
        let pos = self.guard.facing();

        self.visited.insert(self.guard.clone()); // Is there a way to get rid of clone here?

        // Turn if there is an obstacle turn
        if self.obstacles.contains(&pos) {
            self.guard.turn();
            return true;
        }

        // Check that we are not going to step outside the grid
        if !(0..self.width).contains(&(pos.x as usize))
            || !(0..self.height).contains(&(pos.y as usize))
        {
            return false;
        }

        // Check if placing an obstacle infront causes a loop
        if check_loops && !self.visited.iter().any(|guard| guard.loc == pos) {
            // println!("{}", self.visited.len());
            if self.check_loop() {
                self.new_locs.insert(pos.clone());
            }
        }

        // all clear, move forward
        self.guard.forward();
        true
    }
    fn check_loop(&self) -> bool {
        let mut tmp_map = Map {
            height: self.height,
            width: self.width,
            guard: self.guard.clone(),
            new_locs: HashSet::new(),
            visited: HashSet::new(),
            obstacles: self.obstacles.clone(),
        };

        tmp_map.obstacles.insert(tmp_map.guard.facing());

        loop {
            // More than 4 turns or going off edge can't be a loop
            if !tmp_map.step(false) {
                break false;
            }

            // Visited this location in this direction, stuck in loop
            if tmp_map.visited.contains(&tmp_map.guard) || self.visited.contains(&tmp_map.guard) {
                break true;
            }
        }
    }
}

impl FromStr for Map {
    type Err = MapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut obstacles = HashSet::new();
        let mut guard = None;
        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert(Position::from_usize(j, i));
                    }
                    '^' => {
                        guard = Some(Guard::from_cords(j, i));
                    }
                    '.' => (),
                    _ => return Err(MapError::ParseMapError),
                }
            }
        }
        Ok(Map {
            height: s.lines().count(),
            width: s.split_once("\n").unwrap().0.len(),
            guard: guard.unwrap(),
            new_locs: HashSet::new(),
            visited: HashSet::new(),
            obstacles,
        })
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_str = String::new();
        for y in 0..self.height {
            let mut line = vec!['.'; self.width];
            for x in 0..self.width {
                let pos = Position::from_usize(x, y);
                if self.new_locs.contains(&pos) {
                    line[x] = 'O';
                } else if self.guard.loc == pos {
                    line[x] = match self.guard.direction {
                        Direction::North => '^',
                        Direction::South => 'v',
                        Direction::East => '>',
                        Direction::West => '<',
                    }
                } else if self.obstacles.contains(&pos) {
                    line[x] = '#';
                } else if self.visited.iter().any(|guard_pos| guard_pos.loc == pos) {
                    line[x] = '.'
                    // line[x] = match self.guard.direction {
                    //     Direction::North | Direction::South => '|',
                    //     Direction::East | Direction::West => '-',
                    // };
                }
            }

            let line_str: String = line.iter().collect();
            out_str.push_str(&line_str);
            out_str.push('\n');
        }
        write!(f, "{}", out_str)
    }
}
#[derive(Debug)]
enum MapError {
    ParseMapError,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Guard {
    loc: Position,
    direction: Direction,
}

impl Guard {
    fn from_cords(x: usize, y: usize) -> Self {
        Guard {
            loc: Position::from_usize(x, y),
            direction: Direction::North,
        }
    }
}

impl Guard {
    fn facing(&self) -> Position {
        match self.direction {
            Direction::North => Position {
                x: self.loc.x,
                y: self.loc.y - 1,
            },
            Direction::South => Position {
                x: self.loc.x,
                y: self.loc.y + 1,
            },
            Direction::East => Position {
                x: self.loc.x + 1,
                y: self.loc.y,
            },
            Direction::West => Position {
                x: self.loc.x - 1,
                y: self.loc.y,
            },
        }
    }

    fn forward(&mut self) {
        match self.direction {
            Direction::North => self.loc.y -= 1,
            Direction::South => self.loc.y += 1,
            Direction::East => self.loc.x += 1,
            Direction::West => self.loc.x -= 1,
        };
    }

    fn turn(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::South => self.direction = Direction::West,
            Direction::East => self.direction = Direction::South,
            Direction::West => self.direction = Direction::North,
        };
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn from_usize(x: usize, y: usize) -> Self {
        Position {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
