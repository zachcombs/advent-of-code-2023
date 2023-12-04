use std::{
    char,
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    vec,
};

fn main() {
    let mut input_total: u32 = 0;
    if let Ok(lines) = read_lines("inputs/day_1_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                // print!("{} ", line);
                input_total += read_line_for_digits(line);
            }
        }
    }

    println!("Sum: {}", input_total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn concat(vec: Vec<u32>) -> u32 {
    let mut acc = 0;
    for elem in vec {
        acc *= 10;
        acc += elem;
    }
    // println!("{}", acc);
    acc
}

//THESE FUNCTIONS ARE SO BAD BUT IT'S WHAT I THOUGHT OF LATE AT NIGHT
fn check_line_for_number(line: &String, index: usize, char: char) -> Option<u32> {
    let mut substring_hashmap: HashMap<&str, u32> = HashMap::new();

    substring_hashmap.insert("one", 1);
    substring_hashmap.insert("two", 2);
    substring_hashmap.insert("thr", 3);
    substring_hashmap.insert("fou", 4);
    substring_hashmap.insert("fiv", 5);
    substring_hashmap.insert("six", 6);
    substring_hashmap.insert("sev", 7);
    substring_hashmap.insert("eig", 8);
    substring_hashmap.insert("nin", 9);

    let start_index = index;
    let max_length = 3;

    let end_index = start_index + max_length;
    let end_index = std::cmp::min(end_index, line.len());

    let found_number: Option<u32> = char.to_digit(10);
    match found_number {
        Some(number) => number,
        None => {
            if end_index - start_index < 3 {
                return None;
            }

            for (key, value) in substring_hashmap {
                if line[start_index..end_index].contains(key) {
                    return Some(value);
                }
            }

            return None;
        }
    };

    return found_number;
}

fn read_line_for_digits(line: String) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for (index, char) in line.chars().enumerate() {
        let char_to_digit: Option<u32> = check_line_for_number(&line, index, char);
        match char_to_digit {
            Some(number) => number,
            None => continue,
        };

        if first_digit.is_none() {
            first_digit = char_to_digit;
        }
        last_digit = char_to_digit;
    }

    let final_tuple: Vec<u32>;

    if last_digit.is_none() {
        final_tuple = vec![first_digit.unwrap_or_default()];
    } else {
        final_tuple = vec![
            first_digit.unwrap_or_default(),
            last_digit.unwrap_or_default(),
        ];
    }

    return concat(final_tuple);
}
