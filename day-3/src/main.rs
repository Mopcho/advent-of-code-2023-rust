use day_3::solution::part_1;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let solution = part_1(input);

    println!("{:?}", solution);
}
