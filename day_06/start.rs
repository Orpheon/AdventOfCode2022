use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    if let Ok(lines) = read_lines("./bigboy.txt") {
        let start_marker_length = 14;
        for line in lines {
            if let Ok(line_str) = line {
                let mut buffer = VecDeque::<char>::with_capacity(4);
                let mut skip = 0;
                let mut output = 0;
                for (idx, c) in line_str.chars().enumerate() {
                    // println!("Starting character {} at index {}", c, idx);
                    // println!("Buffer: {}", buffer.iter().collect::<String>());
                    if skip > 0 {
                        skip -= 1;
                    }
                    if idx >= start_marker_length - 1 {
                        for i in (0..buffer.len()).rev() {
                            if skip < start_marker_length - i - 1 && c == buffer[i] {
                                skip = start_marker_length - i - 1;
                                // println!("character {} at {} was the same, skipping {}", c, i, skip);
                            }
                        }
                        if skip == 0 {
                            output = idx + 1;
                            // println!("Success!");
                            break;
                        }
                        buffer.pop_back();
                    } else {
                        for i in (0..buffer.len()).rev() {
                            if skip < start_marker_length - i - 1 && c == buffer[i] {
                                skip = start_marker_length - i - 1;
                                // println!("character {} at {} was the same, skipping {}", c, i, skip);
                            }
                        }
                    }
                    buffer.push_front(c);
                }
                println!("Index of first non-repeating {} characters: {}", start_marker_length, output);
            }
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed for entire software is: {:?}", duration);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}