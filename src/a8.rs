/*
Part 2 of the day 8 is quite annoying. Definitely it is not programming puzzle.

It is solvable with LCM when input fits, and it fits. Not coincidentally :/ It can count cycle length for each starting point.

So this program is just brute force solution that can find a solution in a few hours.
*/

use std::collections::{BTreeSet, HashMap};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Left = 0,
    Right = 1,
}

type Navigation = Vec<Direction>;

type NodeId = [char; 3];

type Nodes = Vec<[usize; 2]>;
type NodeMap = HashMap<NodeId, usize>;

fn parse(input: &str) -> (Navigation, Nodes, NodeMap) {
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

    let mut pre_nodes: Vec<[NodeId; 2]> = Vec::new();
    let mut node_map: NodeMap = HashMap::new();
    loop {
        let maybe_line = lines.next();
        if let Some(line) = maybe_line {
            let (source_str, target_str) = line.split_once(" = (").unwrap();
            let source = source_str.trim();
            let binding = target_str.replace(")", "");
            let (target_l, target_r) = binding.split_once(", ").unwrap();
            let node_id = <Vec<char> as TryInto<[char; 3]>>::try_into(source.chars().collect::<Vec<char>>()).unwrap().clone();
            pre_nodes.push([
                <Vec<char> as TryInto<[char; 3]>>::try_into(target_l.chars().collect::<Vec<char>>()).unwrap().clone(),
                <Vec<char> as TryInto<[char; 3]>>::try_into(target_r.chars().collect::<Vec<char>>()).unwrap().clone()
            ]);
            node_map.insert(node_id, pre_nodes.len() - 1);
        } else {
            break;
        }
    }

    let mut nodes: Nodes = vec![];
    for [left, right] in pre_nodes.iter() {
        nodes.push([*node_map.get(left).unwrap(), *node_map.get(right).unwrap()]);
    }

    (navigation, nodes, node_map)
}


fn part_1(input: &str) -> usize {
    let (navigation, nodes, node_map) = parse(input);

    let mut current_node = node_map.get(&['A', 'A', 'A']).unwrap();
    let ending_node = node_map.get(&['Z', 'Z', 'Z']).unwrap();
    for (counter, direction) in navigation.iter().cycle().enumerate() {
        if current_node == ending_node {
            return counter;
        }
        current_node = &nodes[*current_node][*direction as usize];
    }
    unreachable!()
}


fn part_2_lcm(input: &str) -> usize {
    /* assumimg exactly one cycle per starting node - however assertions are there to ensure it is true */

    let (navigation, nodes, node_map) = parse(input);

    let mut current_nodes: Vec<usize> = node_map.iter().filter(|(key, _val)| key[2] == 'A').map(|(_key, val)| *val).collect();
    let ending_nodes: Vec<usize> = node_map.iter().filter(|(key, _val)| key[2] == 'Z').map(|(_key, val)| *val).collect();

    let mut cycles: Vec<(usize, usize, usize, usize)> = vec![];  // current_node_idx, ending_node_idx, navigation_idx, cycle_length

    for (counter, direction) in navigation.iter().cycle().enumerate() {
        let navigation_idx = counter % navigation.len();
        for (current_node_idx, current_node) in current_nodes.iter().enumerate() {
            let maybe_ending_node_idx = ending_nodes.iter().position(|ending_node| ending_node == current_node);
            if let Some(ending_node_idx) = maybe_ending_node_idx {
                let maybe_cycle = cycles
                    .iter()
                    .find(
                        |(l_node_idx, l_ending_node_idx, l_navigation_idx, l_cycle_length)|
                            l_node_idx == &current_node_idx && l_ending_node_idx == &ending_node_idx && l_navigation_idx == navigation_idx
                    );
                if let None = maybe_cycle {
                    cycles.push((current_node_idx, ending_node_idx, navigation_idx, counter));
                    println!("find cycle for {}: {} {} {}", current_node_idx, ending_node_idx, navigation_idx, counter);
                }
            }
        }

        if cycles.len() == current_nodes.len() {
            break;
        }

        current_nodes = current_nodes.iter().map(|idx| nodes[*idx][*direction as usize]).collect();

        if counter % 10_000_000_000 == 0 {
            println!("{}", counter);
        }

        // counter % navigation.len() == 0
        // for benchmarking
        // if counter == 10_000_000_000 {
        //     return 0;
        // }
    }

    let cycle_lengths = cycles
        .iter()
        .map(|(current_node_idx, ending_node_idx, navigation_idx, cycle_length)| cycle_length).collect();
    while (cycle_lengths.iter().min() != cycle_lengths.iter().max()) {
        cycle_length.iter(min)
    }
    0
}

fn part_2_brute_force(input: &str) -> usize {
    let (navigation, nodes, node_map) = parse(input);

    let mut current_nodes: Vec<usize> = node_map.iter().filter(|(key, _val)| key[2] == 'A').map(|(_key, val)| *val).collect();
    let ending_nodes: Vec<usize> = node_map.iter().filter(|(key, _val)| key[2] == 'Z').map(|(_key, val)| *val).collect();

    for (counter, direction) in navigation.iter().cycle().enumerate() {
        if current_nodes.iter().all(|idx| ending_nodes.contains(idx)) {
            return counter;
        }

        current_nodes = current_nodes.iter().map(|idx| nodes[*idx][*direction as usize]).collect();

        if counter % 10_000_000_000 == 0 {
            println!("{}", counter);
        }

        // for benchmarking
        // if counter == 10_000_000_000 {
        //     return 0;
        // }
    }
    unreachable!()
}


fn part_2_unrolled(input: &str) -> usize {
    // only for six starting and ending nodes

    let (navigation, nodes, node_map) = parse(input);

    let mut current_nodes: [usize; 6] = node_map.iter().filter(|(key, _val)| key[2] == 'A').map(|(_key, val)| *val).collect::<Vec<_>>().try_into().unwrap();
    let ending_nodes: [usize; 6] = node_map.iter().filter(|(key, _val)| key[2] == 'Z').map(|(_key, val)| *val).collect::<Vec<_>>().try_into().unwrap();

    for (counter, direction) in navigation.iter().cycle().enumerate() {
        if ending_nodes.contains(&current_nodes[0]) {
            if ending_nodes.contains(&current_nodes[1]) {
                if ending_nodes.contains(&current_nodes[2]) {
                    if ending_nodes.contains(&current_nodes[3]) {
                        if ending_nodes.contains(&current_nodes[4]) {
                            if ending_nodes.contains(&current_nodes[5]) {
                                return counter;
                            }
                        }
                    }
                }
            }
        }

        current_nodes[0] = nodes[current_nodes[0]][*direction as usize];
        current_nodes[1] = nodes[current_nodes[1]][*direction as usize];
        current_nodes[2] = nodes[current_nodes[2]][*direction as usize];
        current_nodes[3] = nodes[current_nodes[3]][*direction as usize];
        current_nodes[4] = nodes[current_nodes[4]][*direction as usize];
        current_nodes[5] = nodes[current_nodes[5]][*direction as usize];

        if counter % 10_000_000_000 == 0 {
            println!("{}", counter);
        }

        // for benchmarking
        // if counter == 10_000_000_000 {
        //     return 0;
        // }
    }
    unreachable!()
}


#[derive(Eq, Hash, PartialEq)]
struct Seen(Direction, BTreeSet<NodeId>);

fn main() {
    let input = read_to_string("input8.txt").unwrap();
    let res1 = part_1(input.as_str());
    let res2 = part_2_brute_force(input.as_str());
    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2_unrolled, part_2_brute_force};

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
    fn part_1_test_1() {
        assert_eq!(part_1(INPUT), 2);
    }

    #[test]
    fn part_1_test_2() {
        assert_eq!(part_1(INPUT2), 6);
    }

    #[test]
    fn part_2_test_1() {
        assert_eq!(part_2_brute_force(INPUT), 2);
    }

    #[test]
    fn part_2_test_2() {
        assert_eq!(part_2_brute_force(INPUT2), 6);
    }

    #[test]
    fn part_2_test_3() {
        assert_eq!(part_2_brute_force(INPUT3), 6);
    }
}