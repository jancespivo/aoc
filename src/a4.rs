use std::collections::HashSet;
use std::fs::read_to_string;

struct Line {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

fn parse_line(line: &str) -> Line {
    let (_, numbers_str) = line.split_once(":").unwrap();
    let (winning_numbers_str, numbers_str) = numbers_str.trim().split_once("|").unwrap();
    let winning_numbers = winning_numbers_str.trim().split(" ").filter_map(|x| x.trim().parse().ok()).collect();
    let numbers = numbers_str.trim().split(" ").filter_map(|x| x.trim().parse().ok()).collect();

    Line {
        winning_numbers,
        numbers,
    }
}

fn points_for_line(line: Line) -> u32 {
    let numbers = line.numbers.iter().collect::<HashSet<_>>();
    let winning_numbers = line.winning_numbers.iter().collect::<HashSet<_>>();
    let intersection = numbers.intersection(&winning_numbers).collect::<Vec<_>>();
    println!("{:?}", numbers);
    println!("{:?}", winning_numbers);
    println!("{:?}", intersection);
    if intersection.len() > 0 {
        2_u32.pow(intersection.len() as u32 - 1)
    } else {
        0
    }
}

fn part_1(cards: &str) -> (u32, u32) {
    (cards.lines().map(|x| parse_line(x)).map(|x| points_for_line(x)).sum(), 0)
}


fn main() {
    let cards = read_to_string("input4.txt").unwrap();
    let (sum1, sum2) = part_1(cards.as_str());
    println!("{} {}", sum1, sum2);
}


#[cfg(test)]
mod tests {
    use crate::{parse_line, part_1, points_for_line};

    const CARDS: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn test() {

        for (line_str, points) in CARDS.lines().zip([8, 2, 2, 1, 0, 0]) {
            let line = parse_line(line_str);
            assert_eq!(points_for_line(line), points);
        }
        //
        // let (sum1, sum2) = part_1(cards);
        // println!("{} {}", sum1, sum2);
        // assert_eq!(sum1, 4361);
        // assert_eq!(sum2, 467835);
    }

    #[test]
    fn test2() {
        let (sum1, sum2) = part_1(CARDS);
        println!("{} {}", sum1, sum2);
        assert_eq!(sum1, 13);
        assert_eq!(sum2, 0);
    }
}