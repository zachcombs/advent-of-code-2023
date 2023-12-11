use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn find_sum_of_shortest_path_of_pairs() {
    let mut universe: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("inputs/day_11_small_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                universe.push(line.chars().collect());
            }
        }
    }

    for row in &universe {
        println!("{:?}", row);
    }
}

fn initialize_input_to_galaxy_map() {}
