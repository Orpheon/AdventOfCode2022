use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut score = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_str) = line {
                let mut iterator = line_str.chars();
                let opponent_move = iterator.next().unwrap() as u8 - b'A';
                let outcome = (iterator.nth(1).unwrap() as u8 - b'X') as i32 - 1;
                let my_move = (opponent_move as i32 + outcome + 3) % 3;
                score += (my_move as i32 + 1) + (outcome+1) * 3;
            }
        }
        println!("Total score following strategy guide: {}", score);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}