use std::cmp::max;
use std::fs::read_to_string;
use std::str::Lines;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Default)]
struct Number {
    value: u32,
    digit_counts: usize,
    end_index: usize,
}

#[derive(Default)]
struct Symbol {
    index: usize,
}

#[derive(Default)]
struct Line {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn parse_line(line: &str) -> Line {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    // let mut value: u32 = 0;
    // let mut digit_counts: i8 = 0;
    let mut number: Number = Number::default();
    for (index, char) in line.chars().enumerate() {
        if let Some(digit) = char.to_digit(10) {
            number.value = number.value * 10 + digit;
            number.digit_counts += 1;
            number.end_index = index;
        } else {
            if number.value > 0 {
                numbers.push(number);
            }
            number = Number::default();
            if char != '.' {
                symbols.push(Symbol { index })
            }
        }
    }
    Line { numbers, symbols }
}

fn get_valid_part_numbers_sum(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> u32 {
    let mut sum: u32 = 0;
    for number in numbers.iter() {
        for symbol in symbols.iter() {
            if symbol.index <= (number.end_index + 1) && symbol.index >= (number.end_index - number.digit_counts) {
                sum += number.value;
                break;
            }
        }
    }
    sum
}

fn part_1(schematic: &str) -> u32 {
    let mut part_numbers_sum = 0;
    let mut previous_line: Line = Line::default();
    for line_str in schematic.lines() {
        let current_line = parse_line(line_str);

        // get numbers in current line where symbol in previous line
        part_numbers_sum += get_valid_part_numbers_sum(&current_line.numbers, &previous_line.symbols);

        // get numbers in previous line where symbol in current line
        part_numbers_sum += get_valid_part_numbers_sum(&previous_line.numbers, &current_line.symbols);

        // get numbers in current line where symbol in current line
        part_numbers_sum += get_valid_part_numbers_sum(&current_line.numbers, &current_line.symbols);

        previous_line = current_line;
    }
    part_numbers_sum
}


fn main() {
    let schematic = read_to_string("input3.txt").unwrap();
    part_1(schematic.as_str());
}


#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn test() {
        let schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part_1(schematic), 4361);
    }
}