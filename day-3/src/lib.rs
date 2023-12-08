pub mod solution {
    fn is_number(char: char) -> bool {
        return char.is_numeric();
    }

    fn is_colliding(
        previous_line: &str,
        current_line: &str,
        next_line: &str,
        start_index: usize,
        end_index: usize,
    ) -> bool {
        let current_line_characters: Vec<char> = current_line.chars().collect();

        let start_index_safe = if start_index == 0 { 0 } else { start_index - 1 };
        let end_index_safe = if end_index == current_line.len() {
            current_line.len()
        } else {
            end_index + 1
        };

        let previous_line_characters = previous_line[start_index_safe..end_index_safe + 1].chars();
        let next_line_characters = next_line[start_index_safe..end_index_safe + 1].chars();
        let current_line_left = current_line_characters
            .get(start_index_safe)
            .unwrap_or(&'.');
        let current_line_right = current_line_characters.get(end_index_safe).unwrap_or(&'.');
        let current_line_characters_to_check = vec![current_line_left, current_line_right];

        for char in previous_line_characters {
            if char != '.' && !char.is_numeric() {
                return true;
            }
        }

        for char in next_line_characters {
            if char != '.' && !char.is_numeric() {
                return true;
            }
        }

        for char in current_line_characters_to_check {
            if *char != '.' && !char.is_numeric() {
                return true;
            }
        }

        return false;
    }

    pub fn part_1(input: String) -> isize {
        let lines: Vec<&str> = input.lines().collect();

        let line_length = lines.get(0).unwrap().len();

        let binding = String::from_utf8(vec![b'.'; line_length]).unwrap();
        let fake_line = binding.as_str();

        let mut sum = 0;

        for (line_index, line) in lines.iter().enumerate() {
            let mut current_number = String::new();

            let line_characters: Vec<char> = line.chars().collect();
            let previous_line = if line_index == 0 {
                fake_line
            } else {
                lines.get(line_index - 1).unwrap_or(&fake_line)
            };
            let current_line = lines.get(line_index).unwrap_or(&fake_line);
            let next_line = lines.get(line_index + 1).unwrap_or(&fake_line);

            for (char_index, char) in line.chars().enumerate() {
                let previous_character = if char_index == 0 {
                    '.'
                } else {
                    *line_characters.get(char_index - 1).unwrap()
                };

                if is_number(char) {
                    current_number.push(char);
                }

                if ((!is_number(char) && is_number(previous_character))
                    || (char_index == line_length - 1) && is_number(char))
                    && is_colliding(
                        previous_line,
                        current_line,
                        &next_line,
                        char_index - current_number.len(),
                        char_index - 1,
                    )
                {
                    sum += current_number.parse::<isize>().unwrap_or(0);

                    println!(
                        "Adding number {}",
                        current_number.parse::<isize>().unwrap_or(0)
                    );

                    current_number = String::from("");
                } else if !is_number(char) && is_number(previous_character) {
                    current_number = String::from("");
                }
            }
        }

        return sum;
    }
}
