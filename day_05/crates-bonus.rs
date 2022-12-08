use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut phase = 0;
    let mut crate_stacks: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(line_str) = line {
                if phase == 0 {
                    let line_characters: Vec<char> = line_str.chars().collect();
                    // Phase 0: Read the crate initial state
                    if crate_stacks.len() == 0 {
                        // First iteration, initialize memory
                        let n_stacks = (line_characters.len() + 1) / 4;
                        crate_stacks = vec![Vec::new(); n_stacks];
                    }
                    if line_characters.len() == 0 {
                        // We've reached the end of phase 0
                        phase = 1;
                        for stack in crate_stacks.iter_mut() {
                            stack.reverse();
                        }
                    } else {
                        for i in 0..crate_stacks.len() {
                            let c = line_characters[4*i+1];
                            if c != ' ' && c as u8 >= b'A' {
                                crate_stacks[i].push(c);
                            }
                        }
                    }
                } else if phase == 1 {
                    // Phase 1: Simulate moving the boxes
                    let sections: Vec<usize> = line_str.split(" ").map(|s| s.parse::<usize>()).filter_map(|x| x.ok()).collect();
                    let origin: &mut Vec<char>;
                    let dest: &mut Vec<char>;
                    if sections[1] == sections[2] {
                        // Null-op, just skip this line
                        continue;
                    } else if sections[1] < sections[2] {
                        let (left, right) = crate_stacks.split_at_mut(sections[2] - 1);
                        origin = &mut left[sections[1] - 1];
                        dest = &mut right[0];
                    } else {
                        let (left, right) = crate_stacks.split_at_mut(sections[1] - 1);
                        origin = &mut right[0];
                        dest = &mut left[sections[2] - 1];
                    }
                    let origin_length = origin.len();
                    dest.extend(origin.drain(origin_length - sections[0] ..));
                }
            }
        }
        let result: String = crate_stacks.iter().map(|stack| stack.last().unwrap()).collect();
        println!("Top elements: {}", result);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}