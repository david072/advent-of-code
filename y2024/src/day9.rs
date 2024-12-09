const INPUT: &str = include_str!("../day9.txt");

#[derive(Clone, Copy)]
enum BlockType {
    Free,
    File(u64),
}

#[derive(Clone, Copy)]
struct Block {
    ty: BlockType,
    position: u64,
    len: u64,
}

fn part1(mut blocks: Vec<Block>) {
    let mut result = 0;
    let mut i = 0usize;
    while i < blocks.len() {
        match blocks[i].ty {
            BlockType::File(id) => {
                let blk = &blocks[i];
                result += (blk.position..blk.position + blk.len).sum::<u64>() * id;
                i += 1;
            }
            BlockType::Free => {
                let mut current_block_len = blocks[i].len;
                let mut position = blocks[i].position;
                while current_block_len > 0 && i < blocks.len() {
                    let last_block = blocks.last_mut().unwrap();
                    match last_block.ty {
                        BlockType::Free => {
                            blocks.pop();
                        }
                        BlockType::File(id) => {
                            let len = current_block_len.min(last_block.len);
                            last_block.len -= len;
                            current_block_len -= len;
                            result += (position..position + len).sum::<u64>() * id;
                            position += len;
                            if last_block.len == 0 {
                                blocks.pop();
                            }
                        }
                    }
                }

                i += 1;
            }
        }
    }

    println!("checksum: {result}");
}

fn part2(mut blocks: Vec<Block>) {
    let mut result = 0u64;
    for i in (0..blocks.len()).rev() {
        if let BlockType::File(id) = blocks[i as usize].ty {
            let len = blocks[i].len;
            if let Some(empty_blk) = blocks[0..i]
                .iter_mut()
                .find(|blk| matches!(blk.ty, BlockType::Free) && blk.len >= len)
            {
                result += (empty_blk.position..empty_blk.position + len).sum::<u64>() * id;
                empty_blk.position += len;
                empty_blk.len -= len;
            } else {
                let blk = &blocks[i];
                result += (blk.position..blk.position + blk.len).sum::<u64>() * id;
            }
        }
    }

    println!("checksum: {result}");
}

fn main() {
    let mut blocks = INPUT
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let len = c.to_digit(10).unwrap() as u64;
            if i % 2 == 0 {
                Block {
                    ty: BlockType::File((i / 2) as u64),
                    position: 0,
                    len,
                }
            } else {
                Block {
                    ty: BlockType::Free,
                    position: 0,
                    len,
                }
            }
        })
        .collect::<Vec<_>>();

    {
        let mut position = 0;
        for blk in &mut blocks {
            blk.position = position;
            position += blk.len;
        }
    }

    part1(blocks.clone());
    part2(blocks);
}
