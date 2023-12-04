pub fn find_sum_of_engine_part_numbers() {
    let mut valid_game_id_total: i32 = 0;
    if let Ok(lines) = read_lines("inputs/day_3_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                valid_game_id_total += find_engine_part_numbers(&line);
            }
        }
    }

    println!("Sum: {}", valid_game_id_total);
}

fn find_engine_part_numbers() {
    
}
