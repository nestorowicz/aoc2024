use std::io::read_to_string;
use std::io::stdin;

#[derive(Debug)]
struct Block {
    position: usize,
    size: usize,
    file_id: usize
}

impl Block {
    fn checksum(self: &Block) -> usize {
        self.file_id * (self.position + self.position + self.size - 1) * self.size / 2
    }
}

fn main() {
    let mut files = parse_files();
    let last_id = files.last().unwrap().file_id;

    for file_id in (0..last_id+1).rev() {
        let (file_index, file) = find_file_by_id(&files, file_id).unwrap();
        let space = find_space(&files, file.size, file_index);
        if space.is_none() {
            continue
        }
        let (index, position) = space.unwrap();
        let mut file = files.remove(file_index);
        file.position = position;
        files.insert(index, file);
    }

    let checksum: usize = files.iter().map(|f| f.checksum()).sum();
    println!("{}", checksum);
}

fn parse_files() -> Vec<Block> {
    let mut files: Vec<Block> =  vec![];
    let mut position: usize = 0;

    read_to_string(stdin()).unwrap().chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as usize)
        .enumerate()
        .for_each(|(i, size)| {
            if i%2 == 0 {
                files.push(Block{position, size, file_id: i/2 });
            }
            position += size
        });

    return files;
}

fn find_file_by_id(files: &Vec<Block>, id: usize) -> Option<(usize, &Block)> {
    return files.iter()
        .rev()
        .enumerate()
        .find(|(_, file)| file.file_id == id)
}


fn find_space(files: &Vec<Block>, size: usize, boundary: usize) -> Option<(usize, usize)> {
    for i in 0..boundary {
        let file_a = &files[i];
        let file_b = &files[i+1];
        let space = file_b.position - (file_a.position + file_a.size);
        if space >= size {
            return Some((i+1, file_a.position + file_a.size))
        }
    }
    return None
}

