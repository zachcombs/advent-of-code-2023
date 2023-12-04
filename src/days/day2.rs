use std::{
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
