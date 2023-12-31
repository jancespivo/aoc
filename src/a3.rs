use std::cmp::max;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::str::FromStr;
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

#[derive(Debug, Clone)]
struct Gear {
    index: usize,
}

#[derive(Default)]
struct Line {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
    gears: Vec<Gear>,
}

struct GearParts(Number, Gear, Number);

impl GearParts {
    fn value(&self) -> u32 {
        self.0.value * self.2.value
    }
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

    #[cfg(not(debug_assertions))]
    fn print(&self, parts: &Vec<Number>, gear_parts: &Vec<GearParts>) {}

    #[cfg(debug_assertions)]
    fn print(&self, parts: &Vec<Number>, gear_parts: &Vec<GearParts>) {
        let mut index = 0;
        let max_index = self.max_index();
        loop {
            if let Some(number) = self.number_at_index(index) {
                print!(
                    "{}",
                    if parts.contains(number) {
                        // if gear_parts.contains(number) {
                        //     number.value.to_string().red()
                        // } else {
                        number.value.to_string().purple()
                        // }
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
                gears.push(Gear { index })
            }
        }
    }
    if number.value > 0 {
        numbers.push(number);
    }
    Line { numbers, symbols, gears }
}

fn is_neighbor(symbol_index: usize, number: &Number) -> bool {
    symbol_index <= (number.end_index + 1) && (symbol_index as i32) >= ((number.start_index() as i32) - 1)
}

fn get_valid_part_numbers<'a>(numbers: &'a Vec<Number>, symbols: &'a Vec<Symbol>) -> Vec<Number> {
    let mut parts: Vec<Number> = vec![];
    for number in numbers.iter() {
        for symbol in symbols.iter() {
            if is_neighbor(symbol.index, &number) {
                parts.push(number.clone());
                break;
            }
        }
    }
    parts
}

fn find_gear_parts(maybe_first_numbers: Option<&Vec<Number>>, middle_line: &Line, maybe_last_numbers: Option<&Vec<Number>>) -> Vec<GearParts> {
    fn find_neighbor_numbers(symbol_index: usize, numbers: &Vec<Number>) -> Vec<Number> {
        let mut number_values: Vec<Number> = vec![];
        for number in numbers.iter() {
            if is_neighbor(symbol_index, &number) {
                number_values.push(number.clone());
            }
        }
        number_values
    }
    let mut gear_parts: Vec<GearParts> = vec![];
    for gear in middle_line.gears.iter() {
        let mut neighbor_numbers: Vec<Number> = vec![];
        if let Some(first_numbers) = maybe_first_numbers {
            neighbor_numbers.append(&mut find_neighbor_numbers(gear.index, &first_numbers));
        }
        neighbor_numbers.append(&mut find_neighbor_numbers(gear.index, &middle_line.numbers));
        if let Some(last_numbers) = maybe_last_numbers {
            neighbor_numbers.append(&mut find_neighbor_numbers(gear.index, &last_numbers));
        }
        if neighbor_numbers.len() == 2 {
            gear_parts.push(GearParts(neighbor_numbers[0].clone(), gear.clone(), neighbor_numbers[1].clone()));
        }
    }
    gear_parts
}

fn part_1(schematic: &str) -> (u32, u32) {
    let mut maybe_previous_previous_line: Option<Line> = None;
    let mut maybe_previous_line: Option<Line> = None;
    let mut parts: Vec<Number> = vec![];
    let mut gear_parts: Vec<GearParts> = vec![];
    for line_str in schematic.lines() {
        let current_line = parse_line(line_str);

        // get numbers in current line where symbol in current line
        parts.append(&mut get_valid_part_numbers(&current_line.numbers, &current_line.symbols));

        if let Some(previous_line) = maybe_previous_line {
            // get numbers in current line where symbol in previous line
            parts.append(&mut get_valid_part_numbers(&current_line.numbers, &previous_line.symbols));
            // get numbers in previous line where symbol in current line
            parts.append(&mut get_valid_part_numbers(&previous_line.numbers, &current_line.symbols));
            // println!("{:?}", z);
            if let Some(previous_previous_line) = maybe_previous_previous_line {
                //  case2 - previous_previous, previous has gear, current
                gear_parts.append(&mut find_gear_parts(Some(&previous_previous_line.numbers), &previous_line, Some(&current_line.numbers)));
                previous_previous_line.print(&parts, &gear_parts);
            } else {
                //  case1 - previous_previous = None, previous has gear, current
                gear_parts.append(&mut find_gear_parts(None, &previous_line, Some(&current_line.numbers)));
            }
            maybe_previous_previous_line = Some(previous_line);
        }

        maybe_previous_line = Some(current_line);
    }

    if let Some(previous_line) = maybe_previous_line {
        if let Some(previous_previous_line) = maybe_previous_previous_line {
            //  case3 - maybe_previous_previous_line, maybe_previous_line has gear
            gear_parts.append(&mut find_gear_parts(Some(&previous_previous_line.numbers), &previous_line, None));
            previous_previous_line.print(&parts, &gear_parts);
        }
        previous_line.print(&parts, &gear_parts);
    }
    (
        parts.iter().map(|x| x.value).sum(),
        gear_parts.iter().map(|x| x.value()).sum()
    )
}


fn main() {
    let schematic = read_to_string("input3.txt").unwrap();
    let (sum1, sum2) = part_1(schematic.as_str());
    println!("{} {}", sum1, sum2);
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
        let (sum1, sum2) = part_1(schematic);
        println!("{} {}", sum1, sum2);
        assert_eq!(sum1, 4361);
        assert_eq!(sum2, 467835);
    }
}