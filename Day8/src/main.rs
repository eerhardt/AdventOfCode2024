use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, Read},
};

type AntennaMap = HashMap<char, Vec<Position>>;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let map = parse_input(&contents);
    let dims = get_map_dimensions(&contents);

    part_one(&map, dims);
    part_two(&map, dims);

    Ok(())
}

fn part_one(map: &AntennaMap, dims: Position) {
    let mut antinode_map = HashSet::new();
    create_antinodes_one(&map, &mut antinode_map);

    let antinode_count = count_antinodes(&antinode_map, dims);
    println!("Antinode count: {antinode_count}");
}

fn create_antinodes_one(antenna_map: &AntennaMap, antinode_map: &mut HashSet<Position>) {
    for (_, antenna_positions) in antenna_map {
        for i in 0..antenna_positions.len() {
            for j in i + 1..antenna_positions.len() {
                insert_antinode_one(antenna_positions[i], antenna_positions[j], antinode_map);
            }
        }
    }
}

fn insert_antinode_one(antenna1: Position, antenna2: Position, antinode_map: &mut HashSet<Position>) {
    let diff_x = antenna1.x - antenna2.x;
    let diff_y = antenna1.y - antenna2.y;

    // if diff_x is less than 0, then adding it to antenna1 will create a point in the opposite direction
    let new_x1 = antenna1.x + diff_x;
    let new_x2 = antenna2.x - diff_x;

    let new_y1 = antenna1.y + diff_y;
    let new_y2 = antenna2.y - diff_y;

    antinode_map.insert(Position {
        x: new_x1,
        y: new_y1,
    });

    antinode_map.insert(Position {
        x: new_x2,
        y: new_y2,
    });
}

fn part_two(map: &AntennaMap, dims: Position) {
    let mut antinode_map = HashSet::new();
    create_antinodes_two(&map, dims, &mut antinode_map);

    let antinode_count = count_antinodes(&antinode_map, dims);
    println!("Antinode with harmonics count: {antinode_count}");
}

fn create_antinodes_two(antenna_map: &AntennaMap, dims: Position, antinode_map: &mut HashSet<Position>) {
    for (_, antenna_positions) in antenna_map {
        for i in 0..antenna_positions.len() {
            for j in i + 1..antenna_positions.len() {
                insert_antinodes_two(antenna_positions[i], antenna_positions[j], dims, antinode_map);
            }
        }
    }
}

fn insert_antinodes_two(antenna1: Position, antenna2: Position, dims: Position, antinode_map: &mut HashSet<Position>) {
    let diff_x = antenna1.x - antenna2.x;
    let diff_y = antenna1.y - antenna2.y;

    let mut current_x1 = antenna1.x;
    let mut current_y1 = antenna1.y;

    while current_x1 >= 0 && current_x1 < dims.x && current_y1 >= 0 && current_y1 < dims.y {
        antinode_map.insert(Position {
            x: current_x1,
            y: current_y1,
        });

        current_x1 += diff_x;
        current_y1 += diff_y;
    }

    let mut current_x2 = antenna2.x;
    let mut current_y2 = antenna2.y;

    while current_x2 >= 0 && current_x2 < dims.x && current_y2 >= 0 && current_y2 < dims.y {
        antinode_map.insert(Position {
            x: current_x2,
            y: current_y2,
        });

        current_x2 -= diff_x;
        current_y2 -= diff_y;
    }
}

fn parse_input(contents: &str) -> AntennaMap {
    let mut map = AntennaMap::new();

    let mut lines = contents.lines();
    let mut done = false;
    let mut current_row = 0;

    while !done {
        match lines.next() {
            Some(line) => {
                parse_line(line, current_row, &mut map);
                current_row += 1;
            }
            None => done = true,
        }
    }

    return map;
}

fn parse_line(line: &str, current_row: i32, map: &mut AntennaMap) {
    let mut current_column = 0;
    for c in line.chars() {
        match c {
            '.' => (),
            _ => {
                let positions = map.entry(c).or_insert(Vec::new());
                positions.push(Position {
                    x: current_column,
                    y: current_row,
                });
            }
        }
        current_column += 1;
    }
}

fn get_map_dimensions(contents: &str) -> Position {
    let rows = contents.lines().count() as i32;
    let columns = contents.lines().nth(0).unwrap().chars().count() as i32;

    Position {
        x: columns,
        y: rows,
    }
}

fn count_antinodes(antinode_map: &HashSet<Position>, dims: Position) -> i32 {
    let mut count = 0;
    for node in antinode_map {
        if node.x >= 0 && node.x < dims.x && node.y >= 0 && node.y < dims.y {
            count += 1;
        }
    }
    return count;
}
