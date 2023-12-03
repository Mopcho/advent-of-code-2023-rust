use std::fs;

use day_1::lib;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let answer = lib::part_1(input);

    println!("{:?}", answer);
}
