use std::{fmt::Display, fs, ops::Add, str::FromStr};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut map = Map::from_str(map_str).unwrap();
    let moves = moves_str.chars().filter_map(|c| match c {
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        _ => None,
    });

    println!("{map}");
    for mv in moves {
        map.move_robot(&mv).unwrap();
    }
    println!("{map}");

    let mut total = 0;
    for (vert, row) in map.contents.iter().enumerate() {
        for (horr, obj) in row.iter().enumerate() {
            match obj {
                Object::Box => total += (100 * vert) + horr,
                _ => continue,
            }
        }
    }
    println!("Final score = {total}")
}

#[derive(Debug, Clone)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn add_direction(&self, dir: &Direction) -> Position {
        match dir {
            Direction::Left => Position {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Position {
                row: self.row,
                col: self.col + 1,
            },
            Direction::Up => Position {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Position {
                row: self.row + 1,
                col: self.col,
            },
        }
    }
}

impl Add<Direction> for &Position {
    type Output = Position;

    fn add(self, dir: Direction) -> Self::Output {
        self.add_direction(&dir)
    }
}
impl Add<&Direction> for Position {
    type Output = Position;

    fn add(self, dir: &Direction) -> Self::Output {
        self.add_direction(dir)
    }
}
impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, dir: Direction) -> Self::Output {
        self.add_direction(&dir)
    }
}

impl Add<&Direction> for &Position {
    type Output = Position;

    fn add(self, dir: &Direction) -> Self::Output {
        self.add_direction(dir)
    }
}

#[derive(Debug)]
struct Map {
    robot: Position,
    contents: Vec<Vec<Object>>,
    width: i32,
    height: i32,
}
impl Map {
    fn move_robot(&mut self, dir: &Direction) -> Result<(), String> {
        if self.make_move(dir, &self.robot.clone())? {
            self.robot = self.robot.clone() + dir;
        }
        Ok(())
    }
    fn make_move(&mut self, dir: &Direction, from: &Position) -> Result<bool, String> {
        let obj = self.get_cell(&(from + dir))?;
        let can_move = match obj {
            Object::Wall => false,
            Object::Box => self.make_move(dir, &(from + dir))?,
            Object::Robot => false,
            Object::Empty => true,
        };

        if can_move {
            self.set_cell(&(from + dir), &self.get_cell(from)?)?;
            self.set_cell(from, &Object::Empty)?;
        };

        Ok(can_move)
    }
    fn set_direct_cell(&mut self, row: i32, col: i32, obj: &Object) -> Result<(), String> {
        return self.set_cell(&Position { row, col }, obj);
    }

    fn set_cell(&mut self, pos: &Position, obj: &Object) -> Result<(), String> {
        if pos.row < 0 || pos.row >= self.height || pos.col < 0 || pos.col >= self.width {
            return Err(String::from("Requested point out of map bounds"));
        }

        self.contents[pos.row as usize][pos.col as usize] = obj.clone();
        Ok(())
    }

    fn get_direct_cell(&self, row: i32, col: i32) -> Result<Object, String> {
        return self.get_cell(&Position { row, col });
    }

    fn get_cell(&self, pos: &Position) -> Result<Object, String> {
        if pos.row < 0 || pos.row >= self.height || pos.col < 0 || pos.col >= self.width {
            return Err(String::from("Requested point out of map bounds"));
        }

        Ok(self.contents[pos.row as usize][pos.col as usize].clone())
    }
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robot = Position { row: 0, col: 0 };
        let mut height: i32 = 0;
        let mut width: i32 = 0;

        let contents: Vec<Vec<Object>> = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                height = row as i32 + 1;
                line.chars()
                    .enumerate()
                    .map(|(col, c)| {
                        width = col as i32 + 1;
                        match c {
                            '#' => Object::Wall,
                            'O' => Object::Box,
                            '@' => {
                                robot = Position {
                                    row: row as i32,
                                    col: col as i32,
                                };
                                Object::Robot
                            }
                            _ => Object::Empty,
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(Map {
            robot,
            contents,
            height,
            width,
        })
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_str = String::new();
        self.contents.iter().for_each(|row| {
            row.iter().for_each(|o| {
                let char = match o {
                    Object::Wall => '#',
                    Object::Box => 'O',
                    Object::Robot => '@',
                    Object::Empty => '.',
                };
                map_str.push(char)
            });
            map_str.push('\n')
        });

        write!(f, "{map_str}")
    }
}
#[derive(Debug, Clone)]
enum Object {
    Wall,
    Box,
    Robot,
    Empty,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => '<',
                Direction::Right => '>',
                Direction::Up => '^',
                Direction::Down => 'v',
            }
        )
    }
}
