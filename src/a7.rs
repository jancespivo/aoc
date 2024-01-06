use std::fs::read_to_string;
use counter::Counter;
use itertools::Itertools;

const CARDS: &str = "23456789TJQKA";

const PART_2_CARDS: &str = "J23456789TQKA";

fn parse(input: &str) -> Vec<(Vec<usize>, Vec<usize>, usize)> {
    let mut hands: Vec<(Vec<usize>, Vec<usize>, usize)> = vec![];

    for line in input.lines() {
        let (cards_str, bid_str) = line.split_once(" ").unwrap();
        let mut hand: Vec<usize> = vec![];
        let mut part_2_ranks: Vec<usize> = vec![];
        for card in cards_str.chars() {
            hand.push(CARDS.find(card).unwrap());
            part_2_ranks.push(PART_2_CARDS.find(card).unwrap())
        }
        hands.push((part_2_ranks, hand, bid_str.parse().unwrap()))
    }
    hands

    // input.lines().map(
    //     |line| line.split_once(" ").unwrap()
    // ).map(
    //     |(cards_str, bid_str)| cards_str.chars().filter_map(
    //         |card| CARDS.find(card)
    //     ).collect()
    // );
}

fn get_kind(hand: &Vec<usize>) -> usize {
    let binding = hand
        .iter()
        .collect::<Counter<_>>()
        .most_common_ordered();
    if binding.len() == 1 {
        return 6;
    }
    let kind_counts = binding[0..=1]
        .iter()
        .map(|(_, count)| count)
        .collect_tuple().unwrap();

    match kind_counts {
        (&4_usize, _) => 5,
        (&3_usize, &2_usize) => 4,
        (&3_usize, _) => 3,
        (&2_usize, &2_usize) => 2,
        (&2_usize, _) => 1,
        _ => 0,
    }
}

fn get_part_2_kind(hand: &Vec<usize>) -> usize {
    // let binding = hand
    //     .iter()
    //     .collect::<Counter<_>>()
    //     .most_common_ordered();
    // if binding.len() == 1 {
    //     return 6;
    // }
    // let kind_counts = binding[0..=1]
    //     .iter()
    //     .map(|(_, count)| count)
    //     .collect_tuple().unwrap();
    //
    // base_match kind_counts {
    //     (&4_usize, _) => 5,
    //     (&3_usize, &2_usize) => 4,
    //     (&3_usize, _) => 3,
    //     (&2_usize, &2_usize) => 2,
    //     (&2_usize, _) => 1,
    //     _ => 0,
    // }
    0
}

fn part_1(input: &str) -> usize {
    let hands = parse(input);

    let mut hands_ranked: Vec<(usize, &Vec<usize>, &usize)> = hands.iter().map(
        |(_, hand, bid)|
            (get_kind(hand), hand, bid)
    ).collect();
    hands_ranked.sort();
    hands_ranked
        .iter()
        .enumerate()
        .map(|(index, (_kind, _hand, bid))| (index + 1) * *bid)
        .sum()
}

fn part_2(input: &str) -> u64 {
    let hands = parse(input);

    let mut hands_ranked: Vec<(usize, &Vec<usize>, &usize)> = hands.iter().map(
        |(part_2_ranks, hand, bid)|
            (get_part_2_kind(hand), part_2_ranks, bid)
    ).collect();
    hands_ranked.sort();
    hands_ranked
        .iter()
        .enumerate()
        .map(|(index, (_kind, _part_2_ranks, bid))| (index + 1) * *bid)
        .sum()
}

fn main() {
    let input = read_to_string("input7.txt").unwrap();
    let res1 = part_1(input.as_str());
    let res2 = part_2(input.as_str());
    println!("{} {}", res1, res2)
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(INPUT),
            vec![
                (vec![1, 0, 8, 1, 11], 765),
                (vec![8, 3, 3, 9, 3], 684),
                (vec![11, 11, 4, 5, 5], 28),
                (vec![11, 8, 9, 9, 8], 220),
                (vec![10, 10, 10, 9, 12], 483),
            ]);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 6440);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 5905);
    }
}