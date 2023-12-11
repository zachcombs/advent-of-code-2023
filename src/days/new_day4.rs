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

pub fn find_sum_of_nested_scratchcard_points() {
    let mut card_count_map: HashMap<u32, u32> = HashMap::new();
    let mut scratchcard_total: u32 = 0;
    if let Ok(lines) = read_lines("inputs/day_4_input.txt") {
        for (index, line) in lines.enumerate() {
            if let Ok(line) = line {
                if card_count_map.get(&(index as u32)).is_some() {
                    find_number_of_points_won_recursive(&line);
                } else {
                    
                }
                card_count_map.insert(index as u32, 1);
                scratchcard_total += find_number_of_points_won_recursive(&line);
            }
        }
    }

    println!("Sum: {}", scratchcard_total);
}

fn find_number_of_points_won_recursive(game: &str) -> u32 {
    let scratchcard_entries: Vec<&str> = game
        .split(" | ")
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .collect();

    let scratchcard_winning_numbers: Vec<&str> = game
        .split(" | ")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();

    let mut number_of_points_won = 0;

    for &entry in &scratchcard_entries {
        for &number in &scratchcard_winning_numbers {
            if entry == number {
                if number_of_points_won == 0 {
                    number_of_points_won = 1
                } else {
                    number_of_points_won *= 2
                }
            }
        }
    }

    number_of_points_won
}
