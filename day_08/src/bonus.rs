use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut heightmap: Vec<Vec<u8>> = Vec::new();
        for line in lines {
            if let Ok(line_str) = line {
                heightmap
                    .push(line_str.chars()
                    .map(|c|
                        c.to_digit(10).expect("Could not convert to number") as u8)
                    .collect());
            }
        }
        let mut best_score = 0;
        // for row in heightmap.iter() {
        //     println!("{:?}, ", row);
        // }
        for house_y in 1_i32..((heightmap.len()-1) as i32) {
            for house_x in 1_i32..((heightmap[0].len()-1) as i32) {
                // println!("At position ({}, {}), value {}", house_x, house_y, heightmap[house_y as usize][house_x as usize]);
                let mut acc = 1;
                for direction in [1_i32, -1_i32].iter() {
                    let mut dist = 0;
                    let mut x = house_x + direction;
                    while x >= 0 && x < heightmap[0].len() as i32 {
                        dist += 1;
                        if heightmap[house_y as usize][x as usize] >= heightmap[house_y as usize][house_x as usize] {
                            break;
                        }
                        x = x + direction;
                    }
                    // println!("Horizontal {}: {}", direction, dist);
                    acc *= dist;
                }
                for direction in [1_i32, -1_i32].iter() {
                    let mut dist = 0;
                    let mut y = house_y + direction;
                    // println!("Starting vert {}", y);
                    while y >= 0 && y < heightmap.len() as i32 {
                        // println!("y is {}, that works", y);
                        dist += 1;
                        // println!("Dist += 1");
                        if heightmap[y as usize][house_x as usize] >= heightmap[house_y as usize][house_x as usize] {
                            break;
                        }
                        y = y + direction;
                    }
                    // println!("Vertical {}: {}", direction, dist);
                    acc *= dist;
                }
                // println!("Acc: {}", acc);
                if acc > best_score {
                    best_score = acc;
                }
            }
        }
        println!("Best scenic score possible: {}", best_score);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}