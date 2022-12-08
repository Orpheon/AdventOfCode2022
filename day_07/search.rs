use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Node {
    pub parent: usize,
    pub children: Vec<usize>,
    pub size: i32,
}


struct Tree {
    pub nodes: Vec<Node>,
}
impl Tree {
    pub fn add_child(&mut self, parent_idx: usize, size: i32) -> usize {
        self.nodes.push(Node{
            parent: parent_idx,
            children: Vec::new(),
            size: size
        });
        let child_idx = self.nodes.len() - 1;
        if child_idx != 0 {
            self.nodes[parent_idx].children.push(child_idx);
        }
        return child_idx;
    }
    pub fn propagate_size(&mut self, idx: usize) {
        // Stop at root
        if idx == 0 {
            return;
        }
        let node = &self.nodes[idx];
        let value = node.size;
        let parent_idx = node.parent;
        let node = &mut self.nodes[parent_idx];
        node.size += value;
    }
    pub fn get_aoc_value(&self, idx: &usize) -> i32 {
        // println!("Starting in node {}", idx);
        let mut result = 0;
        for child in self.nodes[*idx].children.iter() {
            if self.nodes[*child].children.len() > 0 {
                // println!("Found child directory {}", child);
                result += self.get_aoc_value(child);
            }
        }
        if self.nodes[*idx].size <= 100000 {
            // println!("Size is {}, adding self", self.nodes[*idx].size);
            result += self.nodes[*idx].size;
        }
        // println!("Finished node {}", idx);
        return result;
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut tree = Tree {
            nodes: Vec::new(),
        };
        // Hardcode the root directory, and then skip 1 line for the cd into it
        let mut directory_idx = tree.add_child(0, 0);
        for line in lines.skip(1) {
            if let Ok(line_str) = line {
                let input = line_str.split(" ").collect::<Vec<&str>>();
                if input[0] == "$" {
                    // I assume that the person controlling the terminal does a depth-first search
                    // and doesn't ever return somewhere they already were
                    if input[1] == "cd" {
                        if input[2] == ".." {
                            directory_idx = tree.nodes[directory_idx].parent;
                        } else {
                            directory_idx = tree.add_child(directory_idx, 0);
                        }
                    }
                } else {
                    // Just ignore directories, we will care about them when we enter them, otherwise they don't matter
                    if input[0] != "dir" {
                        tree.add_child(directory_idx, input[0].parse().unwrap());
                    }
                }
            }
        }
        // Propagate the sizes of everything up the tree
        for i in (0..tree.nodes.len()).rev() {
            tree.propagate_size(i);
        }

        // for i in 0..tree.nodes.len() {
        //     println!("Size at idx {} ({}): {}", i, tree.nodes[i].children.len() > 0, tree.nodes[i].size);
        // }

        // Run down the tree gathering up the desired metric
        println!("Aggregate size of directories < 100000b, with overlap: {}", tree.get_aoc_value(&0));
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}