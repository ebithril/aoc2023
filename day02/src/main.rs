use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

fn part1(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let lines = content.lines();

    let bag = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    
    let mut result = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        let game = parts[0];

        let mut possible = true;
        for section in parts[1].split(';') {
            for cube in section.split(',') {
                if let Some((_, number, color)) = cube.split(' ').collect_tuple() {
                    if number.parse::<i32>().unwrap() <= bag[color] {
                        continue;
                    }

                    possible = false;
                    break;
                }
                else {
                    panic!("broke");
                }
            }
        }

        if !possible {
            continue;
        }

        result += game.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
    }

    result.to_string()
}

fn part2(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let lines = content.lines();

    let mut result = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        let mut shown = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        for section in parts[1].split(';') {
            for cube in section.split(',') {
                if let Some((_, number, color)) = cube.split(' ').collect_tuple() {
                    let n = number.parse::<i32>().unwrap();

                    if  shown[color] > n {
                        continue;
                    }

                    *shown.get_mut(color).unwrap() = n;
                }
                else {
                    panic!("broke");
                }
            }
        }

        let mut power = 1;
        for (_, n) in shown {
            power *= n;
        }

        result += power;
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
        assert_eq!("8".to_string(), part1("example.txt"));
    }

    #[test]
    fn part2_test() {
        assert_eq!("2286".to_string(), part2("example.txt"));
    }
}
