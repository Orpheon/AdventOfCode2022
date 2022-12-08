use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BinaryHeap;

fn main() {
    let mut elf_reserves = BinaryHeap::new();
    let mut accumulator = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_str) = line {
                if line_str.is_empty() {
                    elf_reserves.push(accumulator);
                    accumulator = 0;
                } else {
                    accumulator += line_str.parse::<i32>().unwrap(); 
                }
            }
        }
    }
    // Regular
    println!("Calories by top: {}", elf_reserves.peek().unwrap());
    // Bonus
    let mut top_3 = elf_reserves.pop().unwrap();
    top_3 += elf_reserves.pop().unwrap();
    top_3 += elf_reserves.pop().unwrap();
    println!("Calories of top 3: {}", top_3);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}