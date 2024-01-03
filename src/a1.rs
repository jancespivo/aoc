mod a2;

use std::fs::read_to_string;
use std::str::Lines;

fn part_1(lines: Lines) {
    let mut answer = 0;
    for line in lines {
        let mut first_digit: Option<u32> = None;
        let mut last_digit = 0;
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                if let None = first_digit {
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
    for (digit, number) in numbers.iter().enumerate() {
        if inp.starts_with(number) {
            return Some(digit as u32);
        }
    }
    None
}

fn part_2(lines: Lines) {
    let mut answer = 0;
    for line in lines {
        let mut first_digit: Option<u32> = None;
        let mut last_digit = 0;
        for (pos, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                if let None = first_digit {
                    first_digit = Some(digit);
                }
                last_digit = digit;
            } else if let Some(digit) = get_str_digit(&line[pos..]) {
                if let None = first_digit {
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
    let inp = read_to_string("input1.txt").unwrap();
    let lines = inp.lines();
    part_2(lines);
}