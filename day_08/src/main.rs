use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn spot_trees_horizontal(heightmap: & Vec<Vec<u8>>, visible_trees: &mut Vec<Vec<u8>>, reversed: bool) {
    for y in 1..heightmap.len()-2 {
        let range = match reversed {
            false => itertools::Either::Left(1..heightmap[0].len()-1),
            true => itertools::Either::Right((1..heightmap[0].len()-1).rev()),
        };
        let border_index = match reversed {
            false => 0,
            true => heightmap[0].len()-1,
        };
        let mut current_height = heightmap[y][border_index];
        for x in range {
            if heightmap[y][x] > current_height {
                current_height = heightmap[y][x];
                visible_trees[y-1][x-1] = 1;
            }
        }
    }
}
pub fn spot_trees_vertical(heightmap: & Vec<Vec<u8>>, visible_trees: &mut Vec<Vec<u8>>, reversed: bool) {
    for x in 1..heightmap[0].len()-2 {
        let range = match reversed {
            false => itertools::Either::Left(1..heightmap[0].len()-1),
            true => itertools::Either::Right((1..heightmap[0].len()-1).rev()),
        };
        let border_index = match reversed {
            false => 0,
            true => heightmap.len()-1,
        };
        let mut current_height = heightmap[border_index][x];
        for y in range {
            if heightmap[y][x] > current_height {
                current_height = heightmap[y][x];
                visible_trees[y-1][x-1] = 1;
            }
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut heightmap: Vec<Vec<u8>> = Vec::new();
        for line in lines {
            if let Ok(line_str) = line {
                heightmap
                    .push(line_str.chars()
                    .map(|c|
                        // + 1 to make comparison easier (0 is not an existing height)
                        c.to_digit(10).expect("Could not convert to number") as u8 + 1)
                    .collect());
            }
        }
        let mut visible_trees: Vec<Vec<u8>> = vec![vec![0; heightmap[0].len()-2]; heightmap.len()-2];
        // for row in heightmap.iter() {
        //     println!("{:?}, ", row);
        // }
        spot_trees_horizontal(&heightmap, &mut visible_trees, false);
        // println!("-");
        // for row in visible_trees.iter() {
        //     println!("{:?}, ", row);
        // }
        spot_trees_horizontal(&heightmap, &mut visible_trees, true);
        // println!("-");
        // for row in visible_trees.iter() {
        //     println!("{:?}, ", row);
        // }
        spot_trees_vertical(&heightmap, &mut visible_trees, false);
        // println!("-");
        // for row in visible_trees.iter() {
        //     println!("{:?}, ", row);
        // }
        spot_trees_vertical(&heightmap, &mut visible_trees, true);
        // println!("-");
        // for row in visible_trees.iter() {
        //     println!("{:?}, ", row);
        // }
        let interior_sum = visible_trees.iter().fold(0, |acc, x| acc + x.iter().fold(0, |inner_acc, x| inner_acc + (*x as u32)));
        // Add edges, which are always visible
        let result = interior_sum + (heightmap.len() as u32) * 2 + (heightmap[0].len() as u32 - 2) * 2;
        println!("Number of visible trees: {}", result);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}