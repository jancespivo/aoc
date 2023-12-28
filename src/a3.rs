use std::cmp::max;
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use std::str::Lines;
use std::str::FromStr;
use strum_macros::EnumString;
use colored::Colorize;

#[derive(Default, Debug, Clone, PartialEq)]
struct Number {
    value: u32,
    digit_counts: usize,
    end_index: usize,
}

impl Number {
    fn start_index(&self) -> usize {
        self.end_index + 1 - self.digit_counts
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    index: usize,
}

#[derive(Debug)]
struct Gear {
    index: usize,
}

#[derive(Default)]
struct Line {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
    gears: Vec<Gear>,
}

impl Line {
    fn symbol_at_index(&self, index: usize) -> Option<&Symbol> {
        for symbol in self.symbols.iter() {
            if symbol.index == index {
                return Some(&symbol);
            }
        }
        None
    }

    fn gear_at_index(&self, index: usize) -> Option<&Gear> {
        for gear in self.gears.iter() {
            if gear.index == index {
                return Some(&gear);
            }
        }
        None
    }

    fn number_at_index(&self, index: usize) -> Option<&Number> {
        for number in self.numbers.iter() {
            if number.start_index() == index {
                return Some(&number);
            }
        }
        None
    }

    fn max_index(&self) -> usize {
        let mut index = 0;
        if self.symbols.len() > 0 {
            index = self.symbols[self.symbols.len() - 1].index;
        }
        if self.numbers.len() > 0 {
            index = max(self.numbers[self.numbers.len() - 1].end_index, index);
        }
        index
    }

    fn print(&self, parts: &Vec<Number>) {
        let mut index = 0;
        let max_index = self.max_index();
        loop {
            if let Some(number) = self.number_at_index(index) {
                print!(
                    "{}",
                    if parts.contains(number) {
                        number.value.to_string().purple()
                    } else {
                        number.value.to_string().normal()
                    }
                );
                index = number.end_index + 1;
            } else if let Some(symbol) = self.symbol_at_index(index) {
                if let Some(gear) = self.gear_at_index(index) {
                    print!("{}", symbol.value.to_string().red());
                } else {
                    print!("{}", symbol.value);
                }
                index += 1;
            } else if index <= max_index {
                print!(".");
                index += 1;
            } else {
                println!("");
                return;
            }
        }
    }
}


fn parse_line(line: &str) -> Line {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    let mut gears: Vec<Gear> = vec![];
    // let mut value: u32 = 0;
    // let mut digit_counts: i8 = 0;
    let mut number: Number = Number::default();  // TODO Option<Number>
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
                symbols.push(Symbol { index, value: char })
            }
            if char == '*' {
                gears.push(Gear {index})
            }
        }
    }
    if number.value > 0 {
        numbers.push(number);
    }
    Line { numbers, symbols, gears }
}

fn get_valid_part_numbers<'a>(numbers: &'a Vec<Number>, symbols: &'a Vec<Symbol>) -> Vec<Number> {
    let mut parts: Vec<Number> = vec![];
    for number in numbers.iter() {
        for symbol in symbols.iter() {
            if symbol.index <= (number.end_index + 1) && (symbol.index as i32) >= ((number.start_index() as i32) - 1) {
                parts.push(number.clone());
                break;
            }
        }
    }
    parts
}

fn part_1(schematic: &str) -> u32 {
    let mut maybe_previous_previous_line: Option<Line> = None;
    let mut maybe_previous_line: Option<Line> = None;
    let mut parts: Vec<Number> = vec![];
    for line_str in schematic.lines() {
        let current_line = parse_line(line_str);

        // get numbers in current line where symbol in current line
        parts.append(&mut get_valid_part_numbers(&current_line.numbers, &current_line.symbols));

        if let Some(previous_line) = maybe_previous_line {
            // get numbers in current line where symbol in previous line
            parts.append(&mut get_valid_part_numbers(&current_line.numbers, &previous_line.symbols));
            // get numbers in previous line where symbol in current line
            parts.append(&mut get_valid_part_numbers(&previous_line.numbers, &current_line.symbols));
            previous_line.print(&parts);
            // let z: Vec<u32> = parts.iter().map(|x| x.value).collect();
            // println!("{:?}", z);
        }
        maybe_previous_line = Some(current_line);
    }

    if let Some(previous_line) = maybe_previous_line {
        previous_line.print(&parts);
        // let z: Vec<u32> = parts.iter().map(|x| x.value).collect();
        // println!("{:?}", z);
    }
    //
    // parts.sort_unstable();
    // parts.dedup();
    // 0
    parts.iter().map(|x| x.value).sum()
}


fn main() {
    let schematic = read_to_string("input3.txt").unwrap();
    let sum = part_1(schematic.as_str());
    println!("{}", sum);
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
        let sum = part_1(schematic);
        println!("{}", sum);
        assert_eq!(sum, 4361);
    }
}