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

pub fn find_sum_of_scratchcard_points() {
    let mut scratchcard_point_total: u32 = 0;
    if let Ok(lines) = read_lines("inputs/day_4_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                scratchcard_point_total += find_number_of_points_won(&line);
            }
        }
    }

    println!("Sum: {}", scratchcard_point_total);
}

fn find_number_of_points_won(game: &str) -> u32 {
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

pub fn find_sum_of_nested_scratchcard_points() {
    let mut card_count_map: HashMap<u32, u32> = HashMap::new();
    if let Ok(lines) = read_lines("inputs/day_4_small_input.txt") {
        for (index, line) in lines.enumerate() {
            if let Ok(line) = line {
                println!("{}", line);
                let my_u32: u32 = index as u32 + 1;
                let reference_to_u32: &u32 = &my_u32;
                card_count_map.insert(reference_to_u32.clone(), 1);
                if let Some(entry) = card_count_map.get_mut(reference_to_u32) {
                    *entry += 0;
                } else {
                    card_count_map.insert(reference_to_u32.clone(), 1);
                }
                for (key, value) in &card_count_map {
                    println!("Key: {}, Value: {}", key, value);
                }
                println!("--------------------------");
                let number_of_times_to_play =
                    if let Some(&value) = card_count_map.get(reference_to_u32) {
                        value
                    } else {
                        0
                    };
                // println!("{} : {}", reference_to_u32, number_of_times_to_play);
                println!(
                    "How many times are we playing?: {}",
                    number_of_times_to_play
                );

                if number_of_times_to_play > 0 {
                    for play in 0..number_of_times_to_play {
                        // println!("Playing Extra Game!: {}", index + 1);
                        find_number_of_points_won_recursive(
                            &line,
                            &mut card_count_map,
                            index as u32,
                        );
                    }
                }

                for (key, value) in &card_count_map {
                    println!("Key: {}, Value: {}", key, value);
                }
                println!("--------------------------")
            }
        }
    }

    let mut total_num_of_cards = 0;

    for value in card_count_map.values() {
        total_num_of_cards += value;
    }

    println!("Sum: {}", total_num_of_cards);
}

fn find_number_of_points_won_recursive(
    game: &str,
    game_map: &mut HashMap<u32, u32>,
    index: u32,
) -> u32 {
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

    let mut num_won: u32 = 0;

    for &entry in &scratchcard_entries {
        for &number in &scratchcard_winning_numbers {
            if entry == number {
                num_won += 1;
            }
        }
    }

    println!("Num Won: {} Index: {}", num_won, index,);

    if num_won > 0 {
        for win in index + 2..=index + num_won + 1 {
            if let Some(entry) = game_map.get_mut(&win) {
                *entry += 1;
            } else {
                game_map.insert(win.clone(), 1);
            }
        }
    }

    num_won
}
