use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use itertools::Itertools;

#[derive(Debug)]
struct Mapping {
    submappings: Vec<Submapping>,
}

impl Mapping {
    fn get_dest_from_source(&self, source: &i64) -> i64 {
        let mut s: i64 = source.clone();
        for submapping in self.submappings.iter() {
            s = submapping.get_dest_from_source(&s);
        }
        s
    }
}


type Seeds = Vec<i64>;

type SeedRanges = Vec<SeedRange>;

struct SeedRange {
    start: i64,
    range: i64,
}

// impl SeedRange {
//     fn get_dest_from_source(&self, mapping: &Mapping) -> i64 {
//         let res: i64 = 0;
//         for seed in self.start..(self.start + self.range) {
//             min(res, mapping.get_dest_from_source(&seed))
//         }
//     }
// }

#[derive(Debug)]
struct Submapping {
    maps: Vec<SourceDestMap>,
}

impl Submapping {
    fn get_dest_from_source(&self, source: &i64) -> i64 {
        for map in self.maps.iter() {
            if let Some(dest) = map.get_dest_from_source(source) {
                return dest;
            }
        }
        source.clone()
    }
}

#[derive(Debug)]
struct SourceDestMap {
    source_start: i64,
    destination_start: i64,
    range_length: i64,
}


impl SourceDestMap {
    fn get_dest_from_source(&self, source: &i64) -> Option<i64> {
        let diff = source - self.source_start;
        if diff >= 0 && (diff < self.range_length) {
            Some(self.destination_start + diff)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> (Seeds, SeedRanges, Mapping) {
    let mut lines = input.lines();
    let first_line = lines.next();
    let (_, seeds_str) = first_line.expect("seed must be here").split_once(": ").unwrap();
    let seeds: Seeds = seeds_str.trim().split(" ").map(|x| x.parse().unwrap()).collect();
    let mut seed_ranges = vec![];
    for (i, start) in seeds.iter().enumerate() {
        for range in seeds[i..i + 1].iter() {
            seed_ranges.push(SeedRange { start: *start, range: *range });
        }
    }
    let _ = lines.next();  // empty line
    let mut mapping: Mapping = Mapping { submappings: vec![] };
    loop {
        let desc = lines.next();  // map description
        if let Some(_) = desc {
            let mut submapping = Submapping { maps: vec![] };
            loop {
                let maybe_line = lines.next();
                if let Some(line) = maybe_line {
                    if line == "" {
                        break;
                    }
                    let (destination_start, source_start, range_length) = line.split(" ").map(|x| x.parse().unwrap()).collect_tuple().unwrap();
                    submapping.maps.push(SourceDestMap { destination_start, source_start, range_length });
                } else {
                    break;
                }
            }
            mapping.submappings.push(submapping);
        } else {
            break;
        }
    }
    (seeds, seed_ranges, mapping)
}


fn part_1(input: &str) -> (i64, i64) {
    let (seeds, seed_ranges, mapping) = parse(input);
    (
        seeds.iter().map(|x| mapping.get_dest_from_source(x)).min().unwrap(),
        seed_ranges.iter().map(
            |x| (x.start..(x.start + x.range)).map(
                |seed| mapping.get_dest_from_source(&seed)
            ).min().unwrap()
        ).min().unwrap(),
    )
}

fn main() {
    let input = read_to_string("input5.txt").unwrap();
    let (res1, res2) = part_1(input.as_str());
    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use crate::{parse, part_1};

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn main_test() {
        let (res1, res2) = part_1(INPUT);
        println!("{} {}", res1, res2);
        assert_eq!(res1, 35);
        assert_eq!(res2, 0);
    }

    #[test]
    fn parse_test() {
        let (seeds, seeds_ranges, mapping) = parse(INPUT);
        println!("{:?} {:?}", seeds, mapping);
    }

    fn parse_test_map() {}

    #[test]
    fn test() {
        let input = read_to_string("input5.txt").unwrap();
        let (res1, res2) = part_1(input.as_str());
        assert_eq!(res1, 35);
        assert_eq!(res2, 0);
    }
}