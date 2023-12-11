use std::{
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

fn expand_universe_on_empty_column_or_row(universe: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut new_universe: Vec<Vec<String>> = Vec::new();

    for row in universe.into_iter().rev() {
        new_universe.insert(0, row.clone());
        if row.iter().all(|point| point == ".") {
            new_universe.insert(0, row.clone());
        }
    }

    let mut transposed_universe: Vec<Vec<_>> = (0..new_universe[0].len())
        .map(|col_index| {
            new_universe
                .iter()
                .map(|row| row[col_index].clone())
                .collect()
        })
        .collect();

    for row in transposed_universe.clone().iter().rev() {
        transposed_universe.insert(0, row.clone());
        if row.iter().all(|point| point == ".") {
            transposed_universe.insert(0, row.to_vec());
        }
    }

    let reverse_transposed_universe: Vec<Vec<_>> = (0..transposed_universe[0].len())
        .map(|col_index| {
            transposed_universe
                .iter()
                .map(|row| row[col_index].clone())
                .collect()
        })
        .collect();

    reverse_transposed_universe
}

fn convert_galaxy_to_number(universe: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut new_universe = universe.clone();
    let mut galaxy_index = 1;

    for row in new_universe.iter_mut() {
        for point in row.iter_mut() {
            if *point == "#" {
                *point = galaxy_index.to_string();
                println!("{}", *point);
                galaxy_index += 1;
            }
        }
    }

    return new_universe;
}

pub fn find_sum_of_shortest_path_of_pairs() {
    let mut universe: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines("inputs/day_11_small_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                universe.push(line.chars().map(|c| c.to_string()).collect());
            }
        }
    }

    let expanded_universe = expand_universe_on_empty_column_or_row(universe);
    let numbafied_universe = convert_galaxy_to_number(&expanded_universe);

    for row in &numbafied_universe {
        println!("{:?}", row);
    }
}

// fn initialize_input_to_galaxy_map() {}
