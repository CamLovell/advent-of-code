use std::{cmp::Ordering, fs};

pub fn run() {
    let disk_map: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .filter_map(|char| char.to_digit(10))
        .map(|num| num as usize)
        .collect();

    let mut disk: Vec<DiskItem> = Vec::with_capacity(disk_map.iter().sum());

    let mut id = 0;
    let mut disk_pos = 0;
    for (i, num) in disk_map.iter().enumerate() {
        if i % 2 == 0 {
            disk.push(DiskItem::Used(File {
                start: disk_pos,
                len: *num,
                id,
            }));
            id += 1;
        } else {
            disk.push(DiskItem::Free(FreeSpace {
                start: disk_pos,
                len: *num,
            }));
        }

        disk_pos += *num;
    }

    show_disk(&disk);
    let mut move_ptr = disk.len() - 1;
    loop {
        match &disk[move_ptr] {
            DiskItem::Free(_) => {
                move_ptr -= 1;
                continue;
            }
            DiskItem::Used(_) => match attempt_move(&mut disk, move_ptr) {
                MoveResult::Exact | MoveResult::Impossible => move_ptr -= 1,
                MoveResult::Partial => (),
            },
        }
        if move_ptr == 0 {
            break;
        }
    }

    let mut check_sum = 0;
    let mut i = 0;
    for item in disk {
        let inc = match item {
            DiskItem::Used(file) => {
                check_sum += file.id * (i..(i + file.len)).sum::<usize>();
                file.len
            }
            DiskItem::Free(space) => space.len,
        };
        i += inc;
    }
    println!("{}", check_sum);
}

fn attempt_move(disk: &mut Vec<DiskItem>, idx: usize) -> MoveResult {
    let to_move = disk[idx].clone();
    for i in 0..idx {
        if let DiskItem::Free(space) = &disk[i] {
            match space.len.cmp(&to_move.len()) {
                Ordering::Equal => {
                    disk[idx] = DiskItem::Free(FreeSpace {
                        start: to_move.start(),
                        len: to_move.len(),
                    });
                    disk[i] = to_move;
                    return MoveResult::Exact;
                }
                Ordering::Greater => {
                    let new_free = DiskItem::Free(FreeSpace {
                        start: disk[i].start() + to_move.len(),
                        len: disk[i].len() - to_move.len(),
                    });
                    disk[idx] = DiskItem::Free(FreeSpace {
                        start: to_move.start(),
                        len: to_move.len(),
                    });
                    disk[i] = to_move;
                    disk.insert(i + 1, new_free);
                    return MoveResult::Partial;
                }
                Ordering::Less => continue,
            }
        }
    }
    MoveResult::Impossible
}

fn show_disk(disk: &Vec<DiskItem>) {
    for item in disk {
        let (ch, len) = match item {
            DiskItem::Used(file) => (file.id.to_string(), file.len),
            DiskItem::Free(space) => (String::from("."), space.len),
        };

        for _ in 0..len {
            print!("{}", ch)
        }
    }
    print!("\n")
}

enum MoveResult {
    Exact,
    Partial,
    Impossible,
}

#[derive(Debug, Clone)]
struct File {
    start: usize,
    len: usize,
    id: usize,
}

#[derive(Debug, Clone)]
struct FreeSpace {
    start: usize,
    len: usize,
}

#[derive(Debug, Clone)]
enum DiskItem {
    Free(FreeSpace),
    Used(File),
}

impl DiskItem {
    fn len(&self) -> usize {
        match self {
            DiskItem::Free(item) => item.len,
            DiskItem::Used(item) => item.len,
        }
    }
    fn start(&self) -> usize {
        match self {
            DiskItem::Free(item) => item.start,
            DiskItem::Used(item) => item.start,
        }
    }
}
