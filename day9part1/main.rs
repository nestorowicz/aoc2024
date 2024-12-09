use std::io::read_to_string;
use std::io::stdin;

macro_rules! debug {
    ($($arg:tt)*) => { if std::env::var("DEBUG").is_ok() { std::println!($($arg)*) } };
}

fn main() {
    let mut disk_map: Vec<u32> = read_to_string(stdin()).unwrap().chars().filter_map(|c| c.to_digit(10)).collect();
    let mut i: usize = 0;
    let mut j: usize = disk_map.len()-1;
    let mut block: u32 = 0;
    let mut checksum: u64 = 0;
    let mut state: String = "".to_string();
    while i <= j {
        debug!("\nmap={:?} i={} j={} disk_map[i]={} disk_map[j]={} checksum={}", disk_map, i, j, disk_map[i], disk_map[j], checksum);
        if is_file(i) {
            checksum += checksum_for_file(file_id(i), disk_map[i], block);
            state += std::iter::repeat(file_id(i)).take(disk_map[i] as usize).map(|n| n.to_string()).collect::<String>().as_str();
            block += disk_map[i];
            debug!("{}", state);
            i += 1;
            continue
        }
        let free_space = disk_map[i];
        let last_file = disk_map[j];
        if free_space == 0 {
            i+=1;
            continue
        }
        if last_file > free_space {
            disk_map[j] -= free_space;
            checksum += checksum_for_file(file_id(j), free_space, block);
            block += free_space;
            state += std::iter::repeat(file_id(j)).take(free_space as usize).map(|n| n.to_string()).collect::<String>().as_str();
            debug!("{}", state);
            i += 1;
        } else {
            disk_map[i] -= last_file;
            disk_map[j] -= last_file; // cosmetics
            checksum += checksum_for_file(file_id(j), last_file, block);
            block += last_file;
            state += std::iter::repeat(file_id(j)).take(last_file as usize).map(|n| n.to_string()).collect::<String>().as_str();
            debug!("{}", state);
            j -= 2;
        }
    }
    println!("{}", checksum);
}

fn is_file(i: usize) -> bool {
    i % 2 == 0
}

fn file_id(i: usize) -> u32 {
    (i / 2) as u32
}

fn checksum_for_file(file_id: u32, blocks: u32, block: u32) -> u64 {
    debug!("checksum_for_file file_id={} blocks={} block={}", file_id, blocks, block);
    let sum_of_block_ids = (block + block + blocks - 1) * blocks / 2;
    (file_id * sum_of_block_ids) as u64
}
