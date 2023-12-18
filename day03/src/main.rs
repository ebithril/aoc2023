use std::fs;

fn part1(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let chars: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    
    let mut result = 0;

    for (y, line) in chars.iter().enumerate() {
        let mut finding = false;
        let mut start = 0;

        for (x, c) in line.iter().enumerate() {
            let is_num = c.is_numeric();
            if !finding && !is_num {
                continue;
            }

            if finding && is_num && x != line.len() - 1 {
                continue;
            }

            if !finding && is_num {
                finding = true;
                start = x;
                continue;
            }

            finding = false;
            let mut found_symbol = false;

            let y_start = (y as i32 - 1).max(0) as usize;
            let y_end = (y + 2).min(chars.len());

            let x_start = (start as i32 - 1).max(0) as usize;
            let x_end = (x + 1).min(line.len());

            for i in y_start..y_end {
                for j in x_start..x_end {
                    let character = chars[i][j];
                    if character.is_numeric() {
                        continue;
                    }

                    if character == '.' {
                        continue;
                    }

                    found_symbol = true;
                    break;
                }
            }

            if !found_symbol {
                continue;
            }

            let mut end = x;
            if is_num {
                end += 1;
            }

            let number: i32 = line[start..end].iter().collect::<String>().parse().unwrap();
            result += number;
        }
    }

    result.to_string()
}

fn part2(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let chars: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    
    let mut result = 0;

    for (y, line) in chars.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '*' {
                continue;
            }

            let x_start = (x as i32 - 1).max(0) as usize;
            let x_end = (x + 2).min(line.len());

            let y_start = (y as i32 - 1).max(0) as usize;
            let y_end = (y + 2).min(line.len());


        }
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
        assert_eq!("4361".to_string(), part1("example.txt"));
    }

    #[test]
    fn part2_test() {
        assert_eq!("467835".to_string(), part2("example.txt"));
    }
}
