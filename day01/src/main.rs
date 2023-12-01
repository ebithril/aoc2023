use std::fs;

fn part1(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let lines = content.lines();
    let mut result = 0;

    for line in lines {
        for c in line.chars() {
            if !c.is_numeric() {
                continue;
            }

            result += c.to_string().parse::<i32>().unwrap() * 10;
            break;
        }

        for c in line.chars().rev() {
            if !c.is_numeric() {
                continue;
            }

            result += c.to_string().parse::<i32>().unwrap();
            break;
        }
    }

    result.to_string()
}

fn part2(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let lines = content.lines();
    let numbers = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut result = 0;
    for line in lines {
        let mut index = line.len();
        let mut current = 0;

        for (i, number) in numbers.iter().enumerate() {
            if let Some(found_index) = line.find(number) {
                if found_index > index {
                    continue;
                }

                index = found_index;
                current = i as i32 + 1;
            }
        }

        for (i, c) in line.chars().enumerate() {
            if i > index {
                break;
            }

            if !c.is_numeric() {
                continue;
            }

            current = c.to_string().parse::<i32>().unwrap();
            break;
        }

        result += current * 10;
        
        index = 0;

        for (i, number) in numbers.iter().enumerate() {
            if let Some(found_index) = line.rfind(number) {
                if found_index < index {
                    continue;
                }

                index = found_index;
                current = i as i32 + 1;
            }
        }

        for (i, c) in line.chars().rev().enumerate() {
            let ai = line.len() - i;
            if ai <= index {
                break;
            }

            if !c.is_numeric() {
                continue;
            }

            current = c.to_string().parse::<i32>().unwrap();
            break;
        }

        result += current;
    }

    result.to_string()
}

fn main() {
    println!("Part1: {}", part1("input.txt"));
    println!("Part2: {}", part2("input.txt"));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        assert_eq!("142".to_string(), part1("example1.txt"));
    }

    #[test]
    fn part2_test() {
        assert_eq!("281".to_string(), part2("example2.txt"));
    }
}
