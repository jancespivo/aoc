use std::cmp::max;
use std::fs::read_to_string;
use std::str::Lines;

enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from_str(inp: &str) -> Result<Self, String> {
        match inp {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err("Color not found".to_string()),
        }
    }
}

fn color_is_possible(color: &Color, num: i8) -> bool {
    let max_num = match color {
        Color::Red => 12,
        Color::Green => 13,
        Color::Blue => 14,
    };
    if num > max_num {
        false
    } else {
        true
    }
}

type SetPossibility = (bool, i8, i8, i8);

fn set_is_possible(raw_set: &str) -> SetPossibility {
    let mut possible = true;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for raw_cubes in raw_set.split(", ") {
        // raw_cubes = "14 red"
        let (num_str, color_str) = raw_cubes.split_once(" ").unwrap();
        let color = Color::from_str(color_str).unwrap();
        let num: i8 = num_str.parse().unwrap();
        match color {
            Color::Red => red = num,
            Color::Green => green = num,
            Color::Blue => blue = num,
        };
        if !color_is_possible(&color, num) {
            possible = false;
        }
    }
    (possible, red, green, blue)
}

type GamePossibility = (bool, i32);

fn game_is_possible(raw_sets: &str) -> GamePossibility {
    let mut game_possible: bool = true;
    let mut max_r: i32 = 0;
    let mut max_g: i32 = 0;
    let mut max_b: i32 = 0;
    for raw_set in raw_sets.split("; ") {
        // raw_set = "14 red, 12 blue"
        let (set_possible, r, g, b) = set_is_possible(raw_set);
        max_r = max(r as i32, max_r);
        max_g = max(g as i32, max_g);
        max_b = max(b as i32, max_b);
        if !set_possible {
            game_possible = false;
        }
    }
    (game_possible, max_r * max_g * max_b)
}

fn get_raw_sets(line: &str) -> (i16, &str) {
    let (game_id, raw_sets) = line.split_once(": ").unwrap();
    let (_, id_s) = game_id.split_once(" ").unwrap();
    let id = id_s.parse::<i16>().unwrap();
    (id, raw_sets)
}

fn part_1(lines: Lines) {
    let mut suma = 0;
    let mut power_suma = 0;
    for line in lines {
        let (id, raw_sets) = get_raw_sets(line);
        let (game_possible, power) = game_is_possible(raw_sets);
        if game_possible {
            suma += id;
        }
        power_suma += power;
    }
    println!("{}", suma);
    println!("{}", power_suma);
}


fn main() {
    let inp = read_to_string("input2.txt").unwrap();
    let lines = inp.lines();
    part_1(lines);
}


#[cfg(test)]
mod tests {
    use crate::a2::part_1;

    #[test]
    fn test_part_1() {
        let lines = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".lines();
        part_1(lines);
    }
}