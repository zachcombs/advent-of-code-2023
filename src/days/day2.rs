use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn find_sum_of_ids() {
    let mut valid_game_id_total: i32 = 0;
    if let Ok(lines) = read_lines("inputs/day_2_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                valid_game_id_total += judge_if_game_is_possible(&line);
            }
        }
    }

    println!("Sum: {}", valid_game_id_total);
}

pub fn find_power_of_minimum_cubes_per_game() {
    let mut total_sum_of_powers: i32 = 0;
    if let Ok(lines) = read_lines("inputs/day_2_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                total_sum_of_powers += find_minimum_num_of_blocks(&line);
            }
        }
    }

    println!("Sum: {}", total_sum_of_powers);
}

fn find_minimum_num_of_blocks(game: &String) -> i32 {
    let game_data: Vec<&str> = game.split(":").collect();

    let mut min_block_hashmap: HashMap<&str, i32> = HashMap::new();

    for reveal in game_data[1].split(";") {
        for cubes_by_color in reveal.split(",") {
            let number_of_cube_of_color: Vec<&str> = cubes_by_color.trim().split(" ").collect();
            match min_block_hashmap.get(number_of_cube_of_color[1]) {
                Some(value) => {
                    if value
                        < number_of_cube_of_color[0]
                            .parse::<i32>()
                            .as_ref()
                            .ok()
                            .unwrap()
                    {
                        min_block_hashmap.insert(
                            number_of_cube_of_color[1],
                            number_of_cube_of_color[0].parse::<i32>().ok().unwrap(),
                        );
                    }
                }
                None => {
                    min_block_hashmap.insert(
                        number_of_cube_of_color[1],
                        number_of_cube_of_color[0].parse::<i32>().ok().unwrap(),
                    );
                }
            }
        }
    }

    let mut hashmap_set_power = 1;

    for value in min_block_hashmap.values() {
        if hashmap_set_power == 0 {
            hashmap_set_power = *value;
        } else {
            hashmap_set_power *= value;
        }
    }

    return hashmap_set_power;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn judge_if_game_is_possible(game: &String) -> i32 {
    let possible_game_restraints = hashmap! {
        "red" => 12,
        "green" => 13,
        "blue" => 14
    };

    let game_data: Vec<&str> = game.split(":").collect();

    for reveal in game_data[1].split(";") {
        for cubes_by_color in reveal.split(",") {
            let number_of_cube_of_color: Vec<&str> = cubes_by_color.trim().split(" ").collect();
            if possible_game_restraints.get(number_of_cube_of_color[1])
                < number_of_cube_of_color[0].parse::<i32>().as_ref().ok()
            {
                return 0;
            }
        }
    }

    let game_id = game
        .split(":")
        .nth(0)
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .ok()
        .unwrap();

    return game_id;
}
