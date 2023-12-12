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

fn expand_universe_on_empty_column_or_row(universe: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut new_universe: Vec<Vec<String>> = Vec::new();

    for row in universe.into_iter().rev() {
        new_universe.insert(0, row.clone());
        if row.iter().all(|point| point == ".") {
            new_universe.insert(0, vec!["!".to_string(); row.len()]);
        }
    }
    for row in &new_universe {
        for element in row {
            print!("{} ", element);
        }
        println!(); // Move to the next line after printing each row
    }

    let mut transposed_universe: Vec<Vec<_>> = (0..new_universe[0].len())
        .map(|col_index| {
            new_universe
                .iter()
                .map(|row| row[col_index].clone())
                .collect()
        })
        .collect();

    for (index, row) in transposed_universe.clone().iter().enumerate().rev() {
        if row.iter().all(|point| point == "." || point == "!") {
            transposed_universe.insert(index, vec!["!".to_string(); row.len()]);
        }
    }
    println!();

    for row in &transposed_universe {
        for element in row {
            print!("{} ", element);
        }
        println!(); // Move to the next line after printing each row
    }

    return transposed_universe;
}

fn convert_galaxy_to_number(universe: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut new_universe = universe.clone();
    let mut galaxy_index = 1;

    for row in new_universe.iter_mut() {
        for point in row.iter_mut() {
            if *point == "#" {
                *point = galaxy_index.to_string();
                galaxy_index += 1;
            }
        }
    }

    return new_universe;
}

fn find_sum_of_paths_between_pairs(universe: Vec<Vec<String>>) {
    let mut map_of_galaxy_coords: IndexMap<String, Vec<i32>> = IndexMap::new();
    let mut sum_of_distances: i32 = 0;

    for (x, row) in universe.iter().enumerate() {
        for (y, point) in row.iter().enumerate() {
            if point != "." {
                map_of_galaxy_coords.insert(point.to_string(), vec![x as i32, y as i32]);
            }
        }
    }

    for num_to_skip in 0..map_of_galaxy_coords.len() {
        let mut iter = map_of_galaxy_coords.iter().skip(num_to_skip);
        let mut base_galaxy: Vec<i32> = vec![0, 0];
        let mut current_galaxy = "";
        while let Some((key, value)) = iter.next() {
            if current_galaxy == "" {
                base_galaxy = value.to_vec();
                current_galaxy = key;
                continue;
            }
            let difference: i32 =
                (base_galaxy[0] - (value[0])).abs() + (base_galaxy[1] - value[1]).abs();
            sum_of_distances += difference;
        }
    }
    println!("This is the sum: {}", sum_of_distances);
}

pub fn find_sum_of_shortest_path_of_pairs_1000000_bigger() {
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
    find_sum_of_paths_between_pairs(numbafied_universe);
}
