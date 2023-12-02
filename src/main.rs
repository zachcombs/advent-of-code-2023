use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    vec,
};

fn main() {
    let mut input_total: u32 = 0;
    if let Ok(lines) = read_lines("day_1_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                println!("{}", line);
                // println!("{}", read_line_for_digits(line));
                input_total += read_line_for_digits(line);
            }
        }
    }

    // println!("{}", input_total);
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
    println!("{}", acc);
    acc
}

fn read_line_for_digits(line: String) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for char in line.chars() {
        let char_to_digit: Option<u32> = char.to_digit(10);
        match char_to_digit {
            Some(number) => number,
            None => continue,
        };

        if first_digit.is_none() {
            first_digit = char_to_digit
        } else {
            last_digit = char_to_digit;
        }
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
    // println!("{}", final_tuple);

    return concat(final_tuple);
}
