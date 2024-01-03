use std::cmp::{max, min};
use std::fs::read_to_string;
use itertools::Itertools;

#[derive(Debug)]
struct Mapping {
    submappings: Vec<Submapping>,
}

impl Mapping {
    fn get_dest_from_source(&self, source: i64) -> i64 {
        let mut s: i64 = source.clone();
        for submapping in self.submappings.iter() {
            s = submapping.get_dest_from_source(s);
        }
        s
    }
}


type Seeds = Vec<i64>;

type SeedRanges = Vec<SeedRange>;

struct SeedRange {
    start: i64,
    end: i64,
}

impl SeedRange {
    fn new(start: i64, range: i64) -> Self {
        Self { start, end: start + range - 1 }
    }
}

#[derive(Debug)]
struct Submapping {
    maps: Vec<SourceDestMap>,
}

impl Submapping {
    fn get_dest_from_source(&self, source: i64) -> i64 {
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
    source_end: i64,
    destination_diff: i64,
}


impl SourceDestMap {
    fn new(destination_start: i64, source_start: i64, range: i64) -> Self {
        Self {
            source_start,
            source_end: source_start + range - 1,
            destination_diff: destination_start - source_start,
        }
    }

    fn get_dest_from_source(&self, source: i64) -> Option<i64> {
        if source >= self.source_start && source <= self.source_end {
            Some(source + self.destination_diff)
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
    for pair in seeds.chunks(2) {
        seed_ranges.push(SeedRange::new(pair[0], pair[1]));
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
                    submapping.maps.push(SourceDestMap::new(destination_start, source_start, range_length));
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


fn part_1(input: &str) -> i64 {
    let (simple_seeds, _, mapping) = parse(input);
    simple_seeds.iter().map(|x| mapping.get_dest_from_source(*x)).min().unwrap()
}

fn part_2(input: &str) -> i64 {
    let (_, mut seed_ranges, mut mapping) = parse(input);

    for submapping in mapping.submappings.iter_mut() {
        seed_ranges.sort_by_key(|x| x.start);
        let mut seeds = seed_ranges.iter_mut();
        let mut new_seeds: Vec<SeedRange> = vec![];
        submapping.maps.sort_by_key(|x| x.source_start);
        let mut maps = submapping.maps.iter();

        let mut maybe_seed = seeds.next();
        let mut maybe_sourcedestmap = maps.next();
        loop {
            if let Some(ref mut seed) = maybe_seed {
                if let Some(sourcedestmap) = maybe_sourcedestmap {
                    if seed.start <= sourcedestmap.source_end { // DS <= CE
                        if seed.end < sourcedestmap.source_start {  // DE < CS
                            new_seeds.push(SeedRange { start: seed.start, end: seed.end }); // DS DE
                        } else {
                            if seed.start < sourcedestmap.source_start { // DS < CS
                                new_seeds.push(SeedRange { start: seed.start, end: sourcedestmap.source_start }); // DS CS
                            }
                            new_seeds.push(SeedRange {
                                start: max(seed.start, sourcedestmap.source_start) + sourcedestmap.destination_diff,
                                end: min(seed.end, sourcedestmap.source_end) + sourcedestmap.destination_diff,
                            });
                        }

                        if sourcedestmap.source_end < seed.end { // CE < DE
                            seed.start = sourcedestmap.source_end + 1;
                            maybe_sourcedestmap = maps.next()
                        } else {
                            maybe_seed = seeds.next();
                        }
                    } else {
                        maybe_sourcedestmap = maps.next()
                    }
                } else {
                    new_seeds.push(SeedRange { start: seed.start, end: seed.end });
                    maybe_seed = seeds.next();
                }
            } else {
                break;
            }
        }
        seed_ranges = new_seeds;
    }

    seed_ranges.iter().map(|x| x.start).min().unwrap()
}

fn main() {
    let input = read_to_string("input5.txt").unwrap();
    let res1 = part_1(input.as_str());
    let res2 = part_2(input.as_str());
    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

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
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 35);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 46);
    }
}