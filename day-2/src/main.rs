use day_2::solution::part_2;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let solution = part_2(input);

    println!("{}", solution);
}
