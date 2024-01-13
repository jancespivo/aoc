use std::fs::read_to_string;

fn parse(input: &str) -> () {
}


fn part_1(input: &str) -> isize {
    parse(input);
    0
 }

fn part_2(input: &str) -> isize {
    parse(input);
    0

}

fn main() {
    let input = read_to_string("inputX.txt").unwrap();
    let res1 = part_1(input.as_str());
    let res2 = part_2(input.as_str());
    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};

    const INPUT: &str = "";

    #[test]
    fn parse_test() {
        assert_eq!(parse(INPUT), ());
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 0);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 0);
    }
}