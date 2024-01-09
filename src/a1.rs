use std::fs::read_to_string;

fn part_1(input: &str) {
    let mut answer = 0;
    for line in input.lines() {
        let mut first_digit: Option<u32> = None;
        let mut last_digit = 0;
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(digit);
                }
                last_digit = digit;
            }
        }
        answer += first_digit.unwrap() * 10 + last_digit;
    }
    println!("{}", answer);
}

fn get_str_digit(inp: &str) -> Option<u32> {
    let numbers: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    numbers.iter().position(|number| inp.starts_with(number)).map(|x| x as u32)
}

fn part_2(input: &str) {
    let mut answer = 0;
    for line in input.lines() {
        let mut first_digit: Option<u32> = None;
        let mut last_digit = 0;
        for (pos, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10).or_else(|| get_str_digit(&line[pos..])) {
                if first_digit.is_none() {
                    first_digit = Some(digit);
                }
                last_digit = digit;
            }
        }
        answer += first_digit.unwrap() * 10 + last_digit;
    }
    println!("{}", answer);
}

fn main() {
    let input = read_to_string("input1.txt").unwrap();
    part_1(&input);
    part_2(&input);
}