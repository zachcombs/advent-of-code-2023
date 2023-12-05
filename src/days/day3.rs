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

pub fn find_sum_of_engine_part_numbers() {
    let mut valid_game_id_total: i32 = 0;
}

fn find_engine_part_numbers(previous_line: &str, line: &str, next_line: &str) {
    println!("{}", previous_line);
    println!("{}", line);
    println!("{}", next_line);
}
