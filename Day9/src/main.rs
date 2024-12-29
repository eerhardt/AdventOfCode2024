use std::{
    fs::File,
    i32,
    io::{self, Read},
};

struct DiskMap {
    blocks: Vec<i32>,
    free_blocks: Vec<FreeBlock>, // only used in part 2
}

#[derive(Copy, Clone)]
struct FreeBlock {
    start: usize,
    len: usize,
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    part_one(&contents);
    part_two(&contents);

    Ok(())
}

fn parse_input(contents: &str) -> DiskMap {
    let mut blocks = Vec::new();
    let mut free_blocks = Vec::new();

    let mut file_block = true;
    let mut block_id = 0;
    for c in contents.chars() {
        let num = c.to_digit(10).unwrap();

        if file_block {
            for _ in 0..num {
                blocks.push(block_id);
            }
            block_id += 1;
        } else {
            free_blocks.push(FreeBlock {
                start: blocks.len(),
                len: num as usize,
            });
            for _ in 0..num {
                blocks.push(-1); // -1 is a free slot
            }
        }

        file_block = !file_block;
    }
    DiskMap {
        blocks,
        free_blocks,
    }
}

fn part_one(contents: &str) {
    let mut map = parse_input(contents);

    // compact the blocks - move the end to the leftmost free space block
    let mut current_end = map.blocks.len() - 1;
    let mut current_begin = 0;
    while current_end > current_begin {
        while map.blocks[current_end] == -1 {
            // skip free spaces
            current_end -= 1;
        }

        while map.blocks[current_begin] != -1 {
            // find the next free space at the beginning
            current_begin += 1;
        }

        if current_end > current_begin {
            map.blocks[current_begin] = map.blocks[current_end];
            map.blocks[current_end] = -1;
        }
    }

    println!("part one checksum: {}", compute_checksum(&map));
}

fn part_two(contents: &str) {
    let mut map = parse_input(contents);

    // compact the blocks - move whole files
    let mut current_end = map.blocks.len() - 1;
    while current_end > 0 {
        // find the next file ID from the end
        while current_end > 0 && map.blocks[current_end] == -1 {
            // skip free spaces
            current_end -= 1;
        }

        if current_end > 0 {
            let file_end = current_end;
            let current_file_id = map.blocks[current_end];

            while current_end > 0 && map.blocks[current_end] == current_file_id {
                // skip the blocks of the same file
                current_end -= 1;
            }
            let file_length = file_end - current_end;

            let free = try_take_first_free_block(&mut map, file_length, current_end);
            if let Some(free) = free {
                for i in 0..file_length {
                    map.blocks[free + i] = map.blocks[current_end + i + 1];
                    map.blocks[current_end + i + 1] = -1;
                }
            }
        }
    }

    println!("part two checksum: {}", compute_checksum(&map));
}

fn try_take_first_free_block(map: &mut DiskMap, len: usize, end: usize) -> Option<usize> {
    for i in 0..map.free_blocks.len() {
        let free_block = map.free_blocks[i];
        if free_block.start >= end {
            return None; // no need to keep looking
        }

        if free_block.len >= len {
            map.free_blocks.remove(i);
            if free_block.len > len {
                let new_free_block = FreeBlock {
                    start: free_block.start + len,
                    len: free_block.len - len,
                };
                map.free_blocks.insert(i, new_free_block);
            }

            return Some(free_block.start);
        }
    }

    None
}

fn compute_checksum(map: &DiskMap) -> i64 {
    let mut checksum: i64 = 0;
    for i in 0..map.blocks.len() {
        let current: i64 = map.blocks[i].into();
        if current != -1 {
            checksum += i as i64 * current;
        }
    }
    checksum
}
