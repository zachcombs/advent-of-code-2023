mod days;

use days::day1::find_sum_of_numbers_and_letters;
use days::day2::{find_power_of_minimum_cubes_per_game, find_sum_of_ids};
use days::day3::find_sum_of_engine_part_numbers;
use days::day4::{find_sum_of_nested_scratchcard_points, find_sum_of_scratchcard_points};

#[macro_use]
extern crate maplit;
fn main() {
    // print!("Here is the output for day 1 part 2 -> ");
    // find_sum_of_numbers_and_letters();

    // print!("Here is the output for day 2 part 1 -> ");
    // find_sum_of_ids();

    // print!("Here is the output for day 2 part 2 -> ");
    // find_power_of_minimum_cubes_per_game();

    // print!("Here is the output for day 3 part 1 -> ");
    // find_sum_of_engine_part_numbers();

    // print!("Here is the output for day 4 part 1 -> ");
    // find_sum_of_scratchcard_points();

    println!("Here is the output for day 4 part 2 -> ");
    find_sum_of_nested_scratchcard_points();
}
