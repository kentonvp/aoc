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

    if size > 0 {
        // Block too big for free space: splits. The remainder is returned in the Option.
        let used = Block { id, size: free.size, free: false };
        let remainder = Block { id, size: size as usize, free: false };
        (used, Some(remainder))
    } else if size < 0 {
        // Block fits into free space. The free space is made into a block in the Option.
        let free =
            Block { id: free.id, size: free.size - used.size, free: true };
        (used.clone(), Some(free))
    } else {
        // Block fits exactly into free space.
        (used.clone(), None)
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

        disk.push(b);
    }

    disk
}

fn next_free_block(disk: &Vec<Block>, start: usize) -> Option<usize> {
    for (i, b) in disk.iter().enumerate().skip(start) {
        if b.free {
            return Some(i);
        }
    }
    None
}

fn prev_used_block(disk: &Vec<Block>, start: usize) -> Option<usize> {
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

fn part2(contents: &str) -> u64 {
    todo!()
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
