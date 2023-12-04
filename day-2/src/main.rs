use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    'game_loop: for (_, line) in input.lines().enumerate() {
        let game_id = get_game_id(line);
        let hands = line.split(": ").skip(1).next().unwrap();

        for draw in hands.split("; ") {
            for col_and_num in draw.split(", ") {
                let (num, col) = col_and_num.split_once(" ").unwrap();

                let number = num.parse::<i32>().unwrap();

                let possible = match col {
                    "red" => number <= 12,
                    "green" => number <= 13,
                    "blue" => number <= 14,
                    _ => panic!("check your input looser"),
                };

                if !possible {
                    continue 'game_loop;
                }
            }
        }

        sum += game_id;
    }

    println!("{}", sum);
}

fn get_game_id(line: &str) -> i32 {
    return line
        .split(":")
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap();
}
