mod days;

use days::day1::find_sum_of_numbers_and_letters;
use days::day2::find_sum_of_ids;

#[macro_use]
extern crate maplit;
fn main() {
    print!("Here is the output for day 1 -> ");
    find_sum_of_numbers_and_letters();

    print!("Here is the outpur for day 2 -> ");
    find_sum_of_ids()
}
