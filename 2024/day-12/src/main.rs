use std::{fs, str::FromStr};

fn main() {
    let map = Map::from_str(&fs::read_to_string("input.txt").unwrap()).unwrap();

    let mut regions: Vec<Region> = Vec::new();

    for (y, row) in map.map.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if regions.iter().any(|r| r.contains(x, y)) {
                continue;
            }
            regions.push(Region::from(&map, Location::from_usize(x, y), *elem))
        }
    }

    println!(
        "{:#?}",
        regions
            .iter()
            .map(|r| r.area * r.perimeter.len())
            .sum::<usize>()
    );
    println!(
        "{}",
        regions
            .iter()
            .map(|r| r.count_sides() * r.area)
            .sum::<usize>()
    );
    // regions
    //     .iter()
    //     .for_each(|r| println!("{} {}", r.plot_type, r.count_sides()))
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Map {
    fn contains(&self, loc: &Location) -> bool {
        loc.x >= 0 && loc.x < self.width && loc.y >= 0 && loc.y < self.height
    }

    fn at(&self, loc: &Location) -> char {
        self.map[loc.y as usize][loc.x as usize]
    }
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<_>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = map.len() as i32;
        let width = map[0].len() as i32;

        Ok(Map { map, height, width })
    }
}

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: Vec<Edge>,
    contents: Vec<Location>,
    plot_type: char,
}

impl Region {
    fn contains(&self, x: usize, y: usize) -> bool {
        self.contents.contains(&Location::from_usize(x, y))
    }
    fn contains_loc(&self, loc: &Location) -> bool {
        self.contents.contains(loc)
    }
    fn from(map: &Map, loc: Location, plot_type: char) -> Self {
        let mut region = Region {
            area: 0,
            perimeter: Vec::new(),
            contents: Vec::new(),
            plot_type,
        };

        region.expand(map, loc);

        region
    }

    fn expand(&mut self, map: &Map, loc: Location) {
        if self.contains_loc(&loc) {
            return;
        }

        self.area += 1;
        self.contents.push(loc.clone());

        //Left
        let next = Location {
            x: loc.x - 1,
            y: loc.y,
        };
        let edge = Edge {
            orrientation: Orrientation::Vertical,
            origin: 1,
            x: loc.x,
            y: loc.y,
        };
        self.check_next(map, next, edge);
        //Right
        let next = Location {
            x: loc.x + 1,
            y: loc.y,
        };
        let edge = Edge {
            orrientation: Orrientation::Vertical,
            origin: -1,
            x: loc.x + 1,
            y: loc.y,
        };
        self.check_next(map, next, edge);
        //Up
        let next = Location {
            x: loc.x,
            y: loc.y - 1,
        };
        let edge = Edge {
            orrientation: Orrientation::Horrizontal,
            origin: 1,
            x: loc.x,
            y: loc.y,
        };
        self.check_next(map, next, edge);
        //Down
        let next = Location {
            x: loc.x,
            y: loc.y + 1,
        };
        let edge = Edge {
            orrientation: Orrientation::Horrizontal,
            origin: -1,
            x: loc.x,
            y: loc.y + 1,
        };
        self.check_next(map, next, edge);
    }

    fn check_next(&mut self, map: &Map, next: Location, edge: Edge) {
        if !map.contains(&next) || map.at(&next) != self.plot_type {
            self.perimeter.push(edge);
        } else {
            self.expand(map, next);
        }
    }

    fn count_sides(&self) -> usize {
        let mut edge_count = 0;

        let mut vert: Vec<_> = self
            .perimeter
            .iter()
            .filter(|edge| edge.orrientation == Orrientation::Vertical)
            .collect();
        vert.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));

        let mut current = Edge {
            orrientation: Orrientation::Vertical,
            origin: -999,
            x: -999,
            y: -999,
        };
        for edge in vert {
            if edge.x != current.x || edge.y - current.y != 1 || edge.origin != current.origin {
                edge_count += 1;
            }
            current = edge.clone();
        }

        let mut horr: Vec<_> = self
            .perimeter
            .iter()
            .filter(|edge| edge.orrientation == Orrientation::Horrizontal)
            .collect();
        horr.sort_by(|a, b| a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x)));

        let mut current = Edge {
            orrientation: Orrientation::Vertical,
            x: -999,
            y: -999,
            origin: -999,
        };
        for edge in horr {
            if edge.y != current.y || edge.x - current.x != 1 || edge.origin != current.origin {
                edge_count += 1;
            }
            current = edge.clone();
        }

        edge_count
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Edge {
    orrientation: Orrientation,
    origin: i32,
    x: i32,
    y: i32,
}

#[derive(PartialEq, Debug, Clone)]
enum Orrientation {
    Horrizontal,
    Vertical,
}

#[derive(PartialEq, Debug, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn from_usize(x: usize, y: usize) -> Self {
        Location {
            x: x as i32,
            y: y as i32,
        }
    }
}
