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

fn expand_universe_on_empty_column_or_row(
    universe: Vec<Vec<String>>,
) -> (Vec<Vec<String>>, HashMap<i32, &'static str>) {
    let new_universe: Vec<Vec<String>> = universe.clone();
    let mut map_of_large_expansions: HashMap<i32, &str> = HashMap::new();

    for (index, row) in universe.into_iter().enumerate() {
        if row.iter().all(|point| point == ".") {
            // if let Some(empty_row) = new_universe.get_mut(index) {
            //     *empty_row = vec!["!".to_string(); row.len()]
            // }
            map_of_large_expansions.insert(index as i32, "row");
        }
    }
    // for row in &new_universe {
    //     for element in row {
    //         print!("{} ", element);
    //     }
    //     println!(); // Move to the next line after printing each row
    // }

    let transposed_universe: Vec<Vec<_>> = (0..new_universe[0].len())
        .map(|col_index| {
            new_universe
                .iter()
                .map(|row| row[col_index].clone())
                .collect()
        })
        .collect();

    for (index, row) in transposed_universe.clone().iter().enumerate() {
        if row.iter().all(|point| point == ".") {
            // if let Some(empty_row) = new_universe.get_mut(index) {
            //     *empty_row = vec!["!".to_string(); row.len()]
            // }
            map_of_large_expansions.insert(index as i32, "column");
        }
    }
    println!();

    // for row in &transposed_universe {
    //     for element in row {
    //         print!("{} ", element);
    //     }
    //     println!(); // Move to the next line after printing each row
    // }
    // println!();

    // for (key, value) in map_of_large_expansions {
    //     println!("Key: {} Value: {}", key, value);
    // }

    return (new_universe, map_of_large_expansions);
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

    // for row in &new_universe {
    //     for element in row {
    //         print!("{} ", element);
    //     }
    //     println!(); // Move to the next line after printing each row
    // }

    return new_universe;
}

fn find_sum_of_paths_between_pairs(
    universe: Vec<Vec<String>>,
    map_of_gaps: HashMap<i32, &'static str>,
) {
    let mut map_of_galaxy_coords: IndexMap<String, Vec<i32>> = IndexMap::new();
    let mut sum_of_distances: u128 = 0;

    for (key, value) in &map_of_gaps {
        println!("{} {}", key, value);
    }

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
            // for row in &universe {
            //     for element in row {
            //         print!("{} ", element);
            //     }
            //     println!(); // Move to the next line after printing each row
            // }
            if current_galaxy == "" {
                base_galaxy = value.to_vec();
                current_galaxy = key;
                continue;
            }
            let mut compounded_distance_x: u128 = 0;
            let start_number_x = std::cmp::min(base_galaxy[0], value[0]);
            let end_number_x = std::cmp::max(base_galaxy[0], value[0]);
            // println!("Start Number: {}", start_number_x);
            // println!("End Number: {}", end_number_x);
            // println!("This is the base galaxy: {:?}", base_galaxy);

            for num in start_number_x + 1..end_number_x {
                if map_of_gaps
                    .get(&num)
                    .is_some_and(|orientation| *orientation == "row")
                {
                    // println!(
                    //     "There is an empty row inbetween {} and {}",
                    //     start_number_x, end_number_x
                    // );
                    compounded_distance_x += 999999;
                }
            }
            // println!("Base Galaxy: {} {:?}", current_galaxy, base_galaxy);
            // println!("Current Compare: {} {:?}", key, value);
            // println!("Distance X: {}", compounded_distance_x);

            let mut compounded_distance_y: u128 = 0;
            let start_number_y = std::cmp::min(base_galaxy[1], value[1]);
            let end_number_y = std::cmp::max(base_galaxy[1], value[1]);

            for num in start_number_y + 1..end_number_y {
                if map_of_gaps
                    .get(&num)
                    .is_some_and(|orientation| *orientation == "column")
                {
                    compounded_distance_y += 999999;
                }
            }
            // println!("Distance Y: {}", compounded_distance_y);

            let diff_x: u128 = (base_galaxy[0] - (value[0])).abs() as u128 + compounded_distance_x;
            let diff_y: u128 = (base_galaxy[1] - value[1]).abs() as u128 + compounded_distance_y;
            // println!("Distance X: {}", diff_x);
            // println!("Distance Y: {}", diff_y);
            sum_of_distances += diff_x + diff_y;
            // println!("------------------------------------------")
        }
        // println!("------------------------------");
    }
    println!("This is the sum: {}", sum_of_distances);
}

pub fn find_sum_of_shortest_path_of_pairs_1000000_bigger() {
    let mut universe: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines("inputs/day_11_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                universe.push(line.chars().map(|c| c.to_string()).collect());
            }
        }
    }

    let expanded_universe = expand_universe_on_empty_column_or_row(universe);
    let numbafied_universe = convert_galaxy_to_number(&expanded_universe.0);
    find_sum_of_paths_between_pairs(numbafied_universe, expanded_universe.1);
}
