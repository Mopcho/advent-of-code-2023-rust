pub mod lib {
    pub fn part_2(input: String) -> u32 {
        let answer = input.lines().map(calculate_line).sum::<u32>();

        return answer;
    }

    fn calculate_line(line: &str) -> u32 {
        let mut characters = (0..line.len()).filter_map(|index| {
            let new_line = &line[index..];

            let char = if new_line.starts_with("one") {
                '1'
            } else if new_line.starts_with("two") {
                '2'
            } else if new_line.starts_with("three") {
                '3'
            } else if new_line.starts_with("four") {
                '4'
            } else if new_line.starts_with("five") {
                '5'
            } else if new_line.starts_with("six") {
                '6'
            } else if new_line.starts_with("seven") {
                '7'
            } else if new_line.starts_with("eight") {
                '8'
            } else if new_line.starts_with("nine") {
                '9'
            } else {
                new_line.chars().next().unwrap()
            };

            char.to_digit(10)
        });

        let first = characters.next().unwrap();

        match characters.last() {
            Some(x) => format!("{}{}", first, x),
            None => format!("{}{}", first, first),
        }
        .parse::<u32>()
        .unwrap()
    }

    pub fn part_1(input: String) -> u32 {
        let answer = input
            .lines()
            .map(|line| {
                let mut characters = line.chars().filter_map(|character| character.to_digit(10));

                let first = characters.next().unwrap();

                match characters.last() {
                    Some(x) => format!("{}{}", first, x),
                    None => format!("{}{}", first, first),
                }
                .parse::<u32>()
                .unwrap()
            })
            .sum::<u32>();

        return answer;
    }
}
