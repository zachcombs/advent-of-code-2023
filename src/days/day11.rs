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

fn expand_universe_on_empty_column_or_row(universe: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_universe: Vec<Vec<char>> = Vec::new();
    for row in universe.into_iter().rev() {
        new_universe.insert(0, row.clone());
        if row.iter().all(|&point| point == '.') {
            // Insert the row at the beginning of the new universe
            new_universe.insert(0, row)
        }
    }

    // let num_of_columns_in_galaxy = universe.get(0).map_or(0, |row| row.len());
    // for col_index in 0..num_of_columns_in_galaxy {
    //     for row in universe {
    //         if let Some(point) = row.get(col_index) {
    //             if *point != '.' {
    //                 break;
    //             }
    //         }
    //         if col_index == num_of_columns_in_galaxy {
    //             universe.insert(x, *row)
    //         }
    //     }
    // }

    return new_universe;
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

    let expanded_universe = expand_universe_on_empty_column_or_row(universe);

    for row in &expanded_universe {
        println!("{:?}", row);
    }
}

fn initialize_input_to_galaxy_map() {}
