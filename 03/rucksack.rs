use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(line_str) = line {
                let byte_str = line_str.as_bytes();
                let half_length = byte_str.len() / 2;
                let left_rucksack: HashSet<u8> = HashSet::from_iter(byte_str.iter().take(half_length).cloned());
                let overlap = byte_str.iter().skip(half_length).filter(|x| left_rucksack.contains(x)).next().unwrap();
                if overlap >= &b'a' {
                    score += (overlap - b'a' + 1) as i32;
                } else {
                    score += (overlap - b'A' + 27) as i32;
                }
            }
        }
        println!("Score: {}", score);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}