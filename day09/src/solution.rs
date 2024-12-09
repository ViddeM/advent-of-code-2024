use std::{collections::HashMap, u32};

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
    input.trim().chars().map(|c| c.to_digit(10).unwrap())
}

fn debug_disk(disk: &Vec<Option<usize>>) {
    let mut s = String::new();
    for l in disk.iter() {
        if let Some(i) = l {
            s.push_str(i.to_string().as_str());
        } else {
            s.push('.');
        }
    }
    println!("{s}");
}

fn compress_disk(disk: &Vec<Option<usize>>) -> Vec<usize> {
    let mut compressed: Vec<usize> = Vec::new();

    let mut end_reader = disk.len() - 1;
    let mut start_reader = 0;
    'outer: while start_reader <= end_reader {
        if let Some(n) = disk[start_reader] {
            compressed.push(n);
            // println!("Using position at {start_reader} {end_reader} {n} (total {compressed:?})");
        } else {
            let n = loop {
                if let Some(i) = disk[end_reader] {
                    end_reader -= 1;
                    break i;
                }
                end_reader -= 1;

                if end_reader == start_reader {
                    break 'outer;
                }
            };

            compressed.push(n);
            // println!("Using char from end {end_reader} {n} (total {compressed:?})");
        }
        start_reader += 1;
    }

    compressed
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = u32>) -> String {
    let mut disk = vec![];

    let mut file_id: usize = 0;
    for (i, num) in input.enumerate() {
        if i % 2 == 0 {
            // is file
            for _ in 0..num {
                disk.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..num {
                disk.push(None);
            }
        }
    }

    // debug_disk(&disk);

    let compressed = compress_disk(&disk);

    // println!("Compressed {compressed:?}");

    let mut checksum = 0;
    for (i, num) in compressed.into_iter().enumerate() {
        checksum += i * num;
    }

    checksum.to_string()
}

fn find_earliest_free_space(
    empty_map: &HashMap<usize, usize>,
    curr_index: usize,
    len: usize,
) -> Option<(usize, usize)> {
    let mut empty_spot: Option<(usize, usize)> = None;
    for (&empty_index, &empty_len) in empty_map.iter() {
        if empty_index >= curr_index {
            continue;
        }

        if empty_len < len {
            continue;
        }

        if let Some((idx, _)) = empty_spot {
            if empty_index > idx {
                continue;
            }
        }

        empty_spot = Some((empty_index, empty_len));
    }
    empty_spot
}

fn compress_disk_2(
    disk: &Vec<Option<usize>>,
    file_map: &HashMap<usize, usize>,
    empty_map: &mut HashMap<usize, usize>,
) -> Vec<Option<usize>> {
    let mut compressed = disk.clone();

    let mut index = disk.len() - 1;
    while index > 0 {
        if let Some(id) = compressed[index] {
            let file_len = file_map.get(&id).expect("File id doesn't exist?");

            if let Some((empty_index, empty_len)) =
                find_earliest_free_space(&empty_map, index, *file_len)
            {
                // Fill in the file in the new spot
                for f in 0..*file_len {
                    compressed[empty_index - empty_len + f + 1] = Some(id);
                }

                // Remove the file from the old spot
                for f in 0..*file_len {
                    compressed[index - f] = None;
                }

                // Update the empty map
                if empty_len == *file_len {
                    empty_map.remove(&empty_index);
                } else {
                    *empty_map
                        .get_mut(&empty_index)
                        .expect("Index no longer exists?") -= file_len;
                }
            }

            let (new_index, overflow) = index.overflowing_sub(*file_len);
            if overflow {
                index = 0;
            } else {
                index = new_index;
            }
        } else {
            index -= 1;
        }
    }

    compressed
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = u32>) -> String {
    let mut disk = vec![];
    let mut file_map = HashMap::new();
    let mut empty_map = HashMap::new();

    let mut file_id: usize = 0;
    for (i, num) in input.enumerate() {
        if i % 2 == 0 {
            // is file
            for _ in 0..num {
                disk.push(Some(file_id));
            }
            file_map.insert(file_id, num as usize);
            file_id += 1;
        } else {
            for _ in 0..num {
                disk.push(None);
            }
            empty_map.insert(disk.len() - 1, num as usize);
        }
    }

    let compressed = compress_disk_2(&disk, &file_map, &mut empty_map);

    let mut checksum = 0;
    for (i, c) in compressed.into_iter().enumerate() {
        if let Some(id) = c {
            checksum += (i as u128) * (id as u128);
        }
    }

    checksum.to_string()
}
