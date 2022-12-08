use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./bonus-input") {
        // TODO: Figure out a better way to do the following three lines
        let temp: Result<Vec<String>, _> = lines.into_iter().collect();
        let lines_vec = temp.unwrap();
        for bags in lines_vec.chunks(3) {
            if let [bag_1, bag_2, bag_3] = bags {
                let bag_1_hashset: HashSet<&u8> = HashSet::from_iter(bag_1.as_bytes().iter());
                let overlap_hashset: HashSet<&u8> = HashSet::from_iter(
                    bag_2.as_bytes().iter().filter(|x| bag_1_hashset.contains(x))
                );
                let badge = bag_3.as_bytes().iter().filter(|x| overlap_hashset.contains(x)).next().unwrap();
                if badge >= &b'a' {
                    score += (badge - b'a' + 1) as i32;
                } else {
                    score += (badge - b'A' + 27) as i32;
                }
            } else {
                panic!("Number of lines is not divisible by 3");
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