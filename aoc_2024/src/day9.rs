// Day 9 //////////////////////////////////////////////////////////////////////
// Block represents each block on the disk. A block can be split up into two
#[derive(Clone, Debug)]
struct Block {
    id: u64,
    size: usize,
    free: bool,
}

// Move takes the used block and tries to move to the free block. Returns a filled block and either
// the remainder, or the free block space left over, or None if the block was filled exactly.
fn move_data(used: &Block, free: &Block) -> (Block, Option<Block>) {
    let id = used.id;
    let size = used.size as i32 - free.size as i32;

    match size {
        0 => {
            // Block fits exactly into free space.
            (used.clone(), None)
        }
        _ if size > 0 => {
            // Block too big for free space: splits. The remainder is returned in the Option.
            let used = Block { id, size: free.size, free: false };
            let remainder = Block { id, size: size as usize, free: false };
            (used, Some(remainder))
        }
        _ => {
            // Block fits into free space. The free space is made into a block in the Option.
            let free =
                Block { id: free.id, size: free.size - used.size, free: true };
            (used.clone(), Some(free))
        }
    }
}

fn decompress(compressed: &str) -> Vec<Block> {
    let mut disk = Vec::with_capacity(compressed.len());

    let mut id = 0;
    for (i, c) in compressed.chars().enumerate() {
        // Weird edge?
        if c.to_digit(10).is_none() {
            break;
        }

        let b = Block {
            id,
            size: c.to_digit(10).expect("Invalid digit") as usize,
            free: i % 2 == 1,
        };

        // Increment id when a free block is createed.
        if b.free {
            id += 1;
        }

        if b.size > 0 {
            disk.push(b);
        }
    }

    disk
}

fn next_free_block(disk: &[Block], start: usize) -> Option<usize> {
    for (i, b) in disk.iter().enumerate().skip(start) {
        if b.free {
            return Some(i);
        }
    }
    None
}

fn prev_used_block(disk: &[Block], start: usize) -> Option<usize> {
    for (i, b) in disk.iter().enumerate().rev().skip(disk.len() - start) {
        if !b.free {
            return Some(i);
        }
    }
    None
}

fn part1(contents: &str) -> u64 {
    let mut disk = decompress(contents);

    let mut free_idx = next_free_block(&disk, 0).expect("No free block");
    let mut used_idx =
        prev_used_block(&disk, disk.len()).expect("No used block");
    while free_idx < used_idx {
        let (used, other) = move_data(&disk[used_idx], &disk[free_idx]);
        // Always update disk with used block.
        disk[free_idx] = used;

        // Update Free or Remaining used block
        if other.is_none() {
            disk.remove(used_idx);
        } else if let Some(block) = other {
            if block.free {
                // Remove the used block first because it will come after the
                // free block.
                disk.remove(used_idx);
                // Insert the remaing block into the correct place in the disk.
                disk.insert(free_idx + 1, block);
            } else {
                // Update the used block with the remaining smaller block.
                disk[used_idx] = block;
            }
        }

        free_idx = next_free_block(&disk, free_idx + 1).expect("No free block");
        used_idx = prev_used_block(&disk, disk.len()).expect("No used block");
    }

    let mut checksum = 0;
    let mut bid = 0;
    for b in disk.iter() {
        if b.free {
            // No more blocks to add.
            break;
        }
        for _ in 0..b.size {
            checksum += bid * b.id;
            bid += 1;
        }
    }

    checksum
}

fn print_disk(disk: &[Block]) {
    print!("[");
    for (i, b) in disk.iter().enumerate() {
        for _ in 0..b.size {
            let token = if b.free { "." } else { &b.id.to_string() };
            print!("{}", token);
        }
        if i < disk.len() - 1 {
            print!("|");
        }
    }
    println!("] {}", disk.len());
}

fn find_free_space_for_block(disk: &[Block], idx: usize) -> Option<usize> {
    let block = &disk[idx];
    for (i, f) in disk.iter().enumerate() {
        if i < idx && f.free && f.size >= block.size {
            return Some(i);
        }
    }
    None
}

fn find_block(disk: &[Block], id: u64) -> usize {
    for (i, b) in disk.iter().enumerate() {
        if b.id == id && !b.free {
            return i;
        }
    }
    panic!("Block not found: {}", id);
}

fn merge_free_spaces(disk: &mut Vec<Block>) {
    let mut i = 1;
    while i < disk.len() {
        if disk[i].free && disk[i - 1].free {
            disk[i - 1].size += disk[i].size;
            disk.remove(i);
        } else {
            i += 1;
        }
    }
}

fn part2(contents: &str) -> u64 {
    let mut disk = decompress(contents);

    let block_idx = prev_used_block(&disk, disk.len()).expect("No used block");
    let mut block_id = disk[block_idx].id;

    loop {
        let used_idx = find_block(&disk, block_id);

        let free_idx = find_free_space_for_block(&disk, used_idx);
        if free_idx.is_none() {
            block_id -= 1;
            if block_id == 0 {
                break;
            }
            continue;
        }

        let free_idx = free_idx.expect("Already check if None");

        let (used, other) = move_data(&disk[used_idx], &disk[free_idx]);
        if other.is_none() {
            disk[free_idx] = used;
            disk[used_idx].free = true;
        } else if let Some(block) = other {
            if block.free {
                disk[free_idx] = used;
                // Remove the used block first because it will come after the
                // free block.
                disk[used_idx].free = true;
                // Insert the remaing block into the correct place in the disk.
                disk.insert(free_idx + 1, block);
            }
        }

        merge_free_spaces(&mut disk);
        if block_id == 0 {
            break;
        }
        block_id -= 1;
    }

    let mut checksum = 0;
    let mut bid = 0;
    for b in disk.iter() {
        if b.free {
            bid += b.size as u64;
            continue;
        }
        for _ in 0..b.size {
            checksum += bid * b.id;
            bid += 1;
        }
    }

    checksum
}

pub fn solve(contents: &str) {
    let time = std::time::Instant::now();
    let res = part1(contents);
    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());

    let time = std::time::Instant::now();
    let res = part2(contents);
    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = r"2333133121414131402";
        assert_eq!(part1(contents), 1928)
    }

    #[test]
    fn test_part2() {
        let contents = r"2333133121414131402";
        assert_eq!(part2(contents), 2858)
    }
}
