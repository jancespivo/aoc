use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Left = 0,
    Right = 1,
}

type Navigation = Vec<Direction>;

type NodeId = [char; 3];

type Directions = [NodeId; 2];
type Graph = HashMap<NodeId, Directions>;

type Nodes = Vec<[usize; 2]>;
type NodeMap = HashMap<NodeId, usize>;

fn parse(input: &str) -> (Navigation, Graph, Nodes, NodeMap) {
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
    let mut pre_nodes: Vec<NodeId> = Vec::new();
    let mut node_map: NodeMap = HashMap::new();
    loop {
        let maybe_line = lines.next();
        if let Some(line) = maybe_line {
            let (source_str, target_str) = line.split_once(" = (").unwrap();
            let source = source_str.trim();
            let binding = target_str.replace(")", "");
            let (target_l, target_r) = binding.split_once(", ").unwrap();
            let node_id = <Vec<char> as TryInto<[char; 3]>>::try_into(source.chars().collect::<Vec<char>>()).unwrap().clone();
            graph.insert(
                node_id,
                [
                    <Vec<char> as TryInto<[char; 3]>>::try_into(target_l.chars().collect::<Vec<char>>()).unwrap().clone(),
                    <Vec<char> as TryInto<[char; 3]>>::try_into(target_r.chars().collect::<Vec<char>>()).unwrap().clone()
                ],
            );
            pre_nodes.push(node_id);
            node_map.insert(node_id, pre_nodes.len() - 1);
        } else {
            break;
        }
    }

    let mut nodes: Nodes = vec![];
    for node_id in pre_nodes.iter() {
        let [left, right] = graph.get(node_id).unwrap();
        nodes.push([*node_map.get(left).unwrap(), *node_map.get(right).unwrap()]);
    }


    (navigation, graph, nodes, node_map)
}


fn part_1(input: &str) -> usize {
    let (navigation, graph, _nodes, _node_map) = parse(input);

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


#[derive(Eq, Hash, PartialEq)]
struct SeenD(Direction, BTreeSet<usize>);

fn part_2(input: &str) -> usize {
    let (navigation, _, nodes, node_map) = parse(input);

    let starting_nodes: Vec<_> = node_map.iter().filter(|(key, _val)| key[2] == 'A').map(|(_key, val)| val).collect();
    let ending_nodes: Vec<_> = node_map.iter().filter(|(key, _val)| key[2] == 'Z').map(|(_key, val)| val).collect();
    let mut current_directions: Vec<[usize; 2]> = starting_nodes.iter().map(|idx| nodes[**idx]).collect();

    let mut seen: HashSet<SeenD> = HashSet::new();

    for (counter, direction) in navigation.iter().cycle().enumerate() {
        let mut targets: Vec<_> = current_directions.iter().map(|directions| directions[*direction as usize]).collect();
        println!("{:?}", targets);
        if ending_nodes.iter().all(|idx| targets.contains(idx)) {
            return counter + 1;
        }

        if !(seen.insert(SeenD(*direction, BTreeSet::from_iter(targets.clone())))) {
            panic!("ALREADY SEEN");
        } else {
            println!("{:?}", seen.len());
        }

        current_directions = targets.iter().map(|idx| nodes[*idx]).collect();
    }
    unreachable!()
}

#[derive(Eq, Hash, PartialEq)]
struct Seen(Direction, BTreeSet<NodeId>);

fn part_2_hashmap(input: &str) -> usize {
    let (navigation, graph, _nodes, _node_map) = parse(input);

    let mut current_directions: Vec<&Directions> = graph.iter().filter(|(key, _val)| key[2] == 'A').map(|(_key, val)| val).collect();
    let mut seen: HashSet<Seen> = HashSet::new();
    for (counter, direction) in navigation.iter().cycle().enumerate() {
        let mut targets: Vec<_> = current_directions.iter().map(|directions| directions[*direction as usize]).collect();
        println!("{:?}", targets);
        if targets.iter().all(|node| node[2] == 'Z') {
            return counter + 1;
        }
        if !(seen.insert(Seen(*direction, BTreeSet::from_iter(targets.clone())))) {
            panic!("ALREADY SEEN");
        } else {
            println!("{:?}", seen.len());
        }

        current_directions = targets.iter().map(|node| graph.get(node).unwrap()).collect();
    }
    unreachable!()
}

fn main() {
    let input = read_to_string("input8.txt").unwrap();
    let res1 = part_1(input.as_str());
    let res2 = part_2(input.as_str());
    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;
    use crate::{parse, part_1, part_2, part_2_hashmap};

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
        assert_eq!(part_2_hashmap(INPUT3), 6);
        assert_eq!(part_2(INPUT3), 6);
    }
}