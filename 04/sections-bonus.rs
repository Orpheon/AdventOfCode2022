use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut n_fully_contained = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(line_str) = line {
                let assignment_pair: Vec<i32> = line_str.split([',', '-']).map(|x| x.parse().unwrap()).collect();
                if !(assignment_pair[0] > assignment_pair[3] || assignment_pair[2] > assignment_pair[1]) {
                    n_fully_contained += 1;
                }
            }
        }
        println!("Assignments fully contained: {}", n_fully_contained);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}