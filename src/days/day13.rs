use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use indexmap::IndexMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn find_sum_of_valley_of_mirrors() {
    let mut patterns: Vec<Vec<Vec<String>>> = Vec::new();
    let mut num_lines_gone_through = 0;
    let mut size_of_patterns: u32 = 0; // 7
    if let Ok(lines) = read_lines("inputs/day_13_small_input.txt") {
        let mut current_pattern: Vec<Vec<String>> = Vec::new();

        for line in lines {
            if let Ok(line) = line {
                // println!("{}", line);
                num_lines_gone_through += 1;
                // println!("{}", num_lines_gone_through);
                if line.is_empty() && size_of_patterns == 0 {
                    size_of_patterns = num_lines_gone_through - 1;
                    patterns.push(current_pattern.clone());
                    current_pattern.clear();
                    num_lines_gone_through = 0;
                } else if num_lines_gone_through == size_of_patterns {
                    current_pattern.push(line.chars().map(|c| c.to_string()).collect());
                    patterns.push(current_pattern.clone());
                    current_pattern.clear();
                    num_lines_gone_through = 0;
                } else {
                    current_pattern.push(line.chars().map(|c| c.to_string()).collect());
                }
            }
        }
    }

    for pattern in patterns {
        println!("{:?}", pattern);
    }
    println!("Num of lines gone through: {}", num_lines_gone_through);
}
