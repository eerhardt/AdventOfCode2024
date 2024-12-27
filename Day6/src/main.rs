use std::{
    fs::File,
    io::{self, Read},
};

const BLOCKED: i8 = 1;
const VISITED: i8 = 2;

const VISITED_UP: i8 = 4;
const VISITED_RIGHT: i8 = 8;
const VISITED_DOWN: i8 = 16;
const VISITED_LEFT: i8 = 32;

type Room = Vec<Vec<i8>>;

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    part_one(&contents);
    part_two(&contents);

    Ok(())
}

struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}
impl Guard {
    fn move_one(&mut self, room: &Room) {
        let next_x = match self.direction {
            Direction::Left => self.x.overflowing_sub(1).0,
            Direction::Right => self.x + 1,
            _ => self.x,
        };

        let next_y = match self.direction {
            Direction::Up => self.y.overflowing_sub(1).0,
            Direction::Down => self.y + 1,
            _ => self.y,
        };

        // check if we are blocked, and if so turn right
        if next_x < room[0].len() && next_y < room.len() && room[next_y][next_x] == BLOCKED {
            self.direction = match self.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            self.x = next_x;
            self.y = next_y;
        }
    }

    fn clone(&self) -> Guard {
        return Guard {
            x: self.x,
            y: self.y,
            direction: self.direction,
        };
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn part_one(contents: &str) {
    let mut room: Room = Vec::new();
    let mut guard = Guard {
        x: 0,
        y: 0,
        direction: Direction::Up,
    };

    parse_input(&contents, &mut guard, &mut room);

    let mut done = false;
    while !done {
        // mark the current location as visited
        room[guard.y][guard.x] = VISITED;

        // move the guard
        guard.move_one(&room);

        // we are done when the guard moves out of the room (going lower than 0 will wrap, so only check for greater)
        done = guard.x >= room[0].len() || guard.y >= room.len();
    }

    println!("Distinct positions: {}", count_visited(&room));
}

fn part_two(contents: &str) {
    let mut orig_room: Room = Vec::new();
    let mut orig_guard = Guard {
        x: 0,
        y: 0,
        direction: Direction::Up,
    };
    parse_input(&contents, &mut orig_guard, &mut orig_room);

    let mut cycles = 0;
    for y in 0..orig_room.len() {
        for x in 0..orig_room[y].len() {
            if x != orig_guard.x || y != orig_guard.y {
                let mut room = duplicate(&orig_room);
                let mut guard = orig_guard.clone();

                // insert an obstruction
                room[y][x] = BLOCKED;
                cycles += detect_cycle(&mut guard, &mut room);
            }
        }
    }

    println!("Possible positions for cycles: {cycles}");
}

fn duplicate(room: &Room) -> Room {
    let mut result: Room = Vec::new();
    for i in 0..room.len() {
        result.push(room[i].clone());
    }
    return result;
}

fn parse_input(contents: &str, guard: &mut Guard, room: &mut Room) {
    let mut lines = contents.lines();
    let mut done = false;
    let mut current_row = 0;
    while !done {
        match lines.next() {
            Some(line) => {
                room.push(create_room_row(guard, current_row, line));
                current_row += 1;
            }
            None => done = true,
        }
    }
}

fn create_room_row(guard: &mut Guard, row: usize, line: &str) -> Vec<i8> {
    let mut result = Vec::new();
    let mut current_column = 0;
    for c in line.chars() {
        result.push(match c {
            '.' => 0,
            '#' => BLOCKED,
            '^' => {
                guard.x = current_column;
                guard.y = row;
                guard.direction = Direction::Up;
                0
            }
            // could do other directions here
            _ => panic!("unexpected char"),
        });
        current_column += 1;
    }

    return result;
}

fn count_visited(room: &Room) -> i32 {
    let mut result = 0;
    for i in 0..room.len() {
        for j in 0..room[i].len() {
            if room[i][j] == VISITED {
                result += 1;
            }
        }
    }
    return result;
}

fn detect_cycle(guard: &mut Guard, room: &mut Room) -> i32 {
    let mut cycle = false;
    let mut done = false;
    while !done {
        // mark the current location as visited
        room[guard.y][guard.x] |= match guard.direction {
            Direction::Up => VISITED_UP,
            Direction::Right => VISITED_RIGHT,
            Direction::Down => VISITED_DOWN,
            Direction::Left => VISITED_LEFT,
        };

        // move the guard
        guard.move_one(&room);

        // (going lower than 0 will wrap, so only check for greater)
        let out_of_room = guard.x >= room[0].len() || guard.y >= room.len();

        if !out_of_room {
            // we are in a cycle if the guard is in the same position as they have been before
            let mask = match guard.direction {
                Direction::Up => VISITED_UP,
                Direction::Right => VISITED_RIGHT,
                Direction::Down => VISITED_DOWN,
                Direction::Left => VISITED_LEFT,
            };
            cycle = room[guard.y][guard.x] & mask == mask;
        }

        // we are done when the guard moves out of the room (going lower than 0 will wrap, so only check for greater)
        // or we are in a cylce
        done = out_of_room || cycle;
    }
    if cycle {
        return 1;
    } else {
        return 0;
    }
}
