use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left = 0,
    Right = 1,
}

type Navigation = Vec<Direction>;
type Graph = HashMap<[char; 3], [[char; 3]; 2]>;

fn parse(input: &str) -> (Navigation, Graph) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let navigation: Navigation = first_line.chars().map(
        |char|
            match char {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            }
    )
        .collect();

    lines.next(); // empty line

    let mut graph: Graph = HashMap::new();
    loop {
        let maybe_line = lines.next();
        if let Some(line) = maybe_line {
            let (source_str, target_str) = line.split_once(" = (").unwrap();
            let source = source_str.trim();
            let binding = target_str.replace(")", "");
            let (target_l, target_r) = binding.split_once(", ").unwrap();
            graph.insert(
                <Vec<char> as TryInto<[char;3]>>::try_into(source.chars().collect::<Vec<char>>()).unwrap().clone(),
                [
                    <Vec<char> as TryInto<[char;3]>>::try_into(target_l.chars().collect::<Vec<char>>()).unwrap().clone(),
                    <Vec<char> as TryInto<[char;3]>>::try_into(target_r.chars().collect::<Vec<char>>()).unwrap().clone()
                ],
            );
        } else {
            break;
        }
    }
    (navigation, graph)
}


fn part_1(input: &str) -> usize {
    let (navigation, graph) = parse(input);
    let mut current_directions = graph.get(&['A', 'A', 'A']).unwrap();
    for (counter, direction) in navigation.iter().cycle().enumerate() {
        let target = current_directions[*direction as usize];
        if target == ['Z', 'Z', 'Z'] {
            return counter + 1;
        }
        current_directions = graph.get(&target).unwrap();
    }
    unreachable!()
}

fn part_2(input: &str) -> u64 {
    parse(input);
    0
}

fn main() {
    let input = read_to_string("input8.txt").unwrap();
    let res1 = part_1(input.as_str());
    let res2 = part_2(input.as_str());
    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};

    const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";


    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn parse_test() {
        parse(INPUT);
        // assert_eq!(parse(INPUT), ());
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 2);
        assert_eq!(part_1(INPUT2), 6);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT3), 6);
    }
}