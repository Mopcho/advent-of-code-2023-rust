use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let answer = input
        .lines()
        .map(|line| {
            let mut characters = line.chars().filter_map(|character| character.to_digit(10));

            let first = characters.next().unwrap();

            let second = characters.next();

            match second {
                Some(x) => format!("{}{}", first, x),
                None => format!("{}{}", first, first),
            }
            .parse::<u32>()
            .unwrap()
        })
        .sum::<u32>();

    println!("{:?}", answer);
}
