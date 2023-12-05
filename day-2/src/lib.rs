pub mod solution {
    pub fn part_1(input: String) -> usize {
        let mut sum = 0;

        'game_loop: for (index, line) in input.lines().enumerate() {
            let game_id = index + 1;
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

        sum
    }

    pub fn part_2(input: String) -> i32 {
        let mut sum = 0;

        for (_, line) in input.lines().enumerate() {
            let hands = line.split(": ").skip(1).next().unwrap();
            let mut red: Vec<i32> = vec![];
            let mut green: Vec<i32> = vec![];
            let mut blue: Vec<i32> = vec![];
            for draw in hands.split("; ") {
                for col_and_num in draw.split(", ") {
                    let (num, col) = col_and_num.split_once(" ").unwrap();

                    let number = num.parse::<i32>().unwrap();

                    match col {
                        "red" => {
                            red.push(number);
                        }
                        "green" => {
                            green.push(number);
                        }
                        "blue" => {
                            blue.push(number);
                        }
                        _ => panic!("check your input looser"),
                    };
                }
            }

            let red_answer = match red.iter().max() {
                Some(x) => x,
                None => &0,
            };

            let green_answer = match green.iter().max() {
                Some(x) => x,
                None => &0,
            };

            let blue_answer = match blue.iter().max() {
                Some(x) => x,
                None => &0,
            };

            sum += red_answer * green_answer * blue_answer;
        }

        sum
    }
}
