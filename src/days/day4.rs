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

pub fn find_sum_of_scratchcard_points() {
    let mut scratchcard_point_total: i32 = 0;
    if let Ok(lines) = read_lines("inputs/day_4_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                scratchcard_point_total += find_number_of_points_won(&line);
            }
        }
    }

    println!("Sum: {}", scratchcard_point_total);
}

fn find_number_of_points_won(game: &str) -> i32 {
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
