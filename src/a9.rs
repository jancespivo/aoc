use std::fs::read_to_string;

fn parse(input: &str) -> impl Iterator<Item=Vec<isize>> + '_ {
    input
        .lines()
        .map(
            |line| line.split(" ").map(|x| x.parse().unwrap()).collect()
        )
}


fn get_polynom_coeficients(count: &usize) -> Vec<isize> {
    let mut previous = Vec::new();
    for _ in 0..*count {
        let mut new: Vec<_> = previous.iter().map(|x| -x).collect();
        new.push(1);
        for idx in 1..new.len() {
            new[idx] = new[idx] + previous[idx - 1]
        }
        previous = new;
    }
    previous
}

fn part_1(input: &str) -> isize {
    parse(input).map(
        |line|
        get_polynom_coeficients(&line.len()).iter().zip(line).map(|(coef, num)| coef * num).sum::<isize>()
    ).sum::<isize>()
}

fn part_2(input: &str) -> isize {
    parse(input).map(
        |line|
        get_polynom_coeficients(&line.len()).iter().zip(line.iter().rev()).map(|(coef, num)| coef * num).sum::<isize>()
    ).sum::<isize>()
}

fn main() {
    let input = read_to_string("input9.txt").unwrap();
    let res1 = part_1(input.as_str());
    let res2 = part_2(input.as_str());
    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    // #[test]
    // fn parse_test() {
    //     assert_eq!(parse(INPUT), ());
    // }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 114);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 2);
    }
}