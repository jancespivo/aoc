use std::fs::read_to_string;

#[derive(PartialEq, Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse(input: &str) -> (Vec<Race>, Race) {
    let lines: Vec<&str> = input.lines().collect();
    let line_times: &str = lines[0];
    let line_distances: &str = lines[1];
    let (_, times_str) = line_times.split_once(":").unwrap();
    let (_, distances_str) = line_distances.split_once(":").unwrap();
    fn parse_line(x: &str) -> impl Iterator<Item=u64> + '_ { x.trim().split(" ").map(|x| x.trim()).filter(|x| *x != "").map(|x| x.parse().unwrap()) }
    let times = parse_line(times_str);
    let distances = parse_line(distances_str);


    let time = times_str.replace(" ", "").parse().unwrap();
    let distance = distances_str.replace(" ", "").parse().unwrap();

    (
        std::iter::zip(times, distances).map(|(time, distance)| Race { time, distance }).collect(),
        Race { time, distance }
    )
}


fn part_1(input: &str) -> u64 {
    let (races, _) = parse(input);
    races.iter().map(
        |race| (1..race.time).map(|power_time| ((race.time - power_time) * power_time) > race.distance).filter(|x| *x).collect::<Vec<_>>().len() as u64
    ).product()
}

fn part_2(input: &str) -> u64 {
    let (_, race) = parse(input);
    (1..race.time).map(|power_time| ((race.time - power_time) * power_time) > race.distance).filter(|x| *x).collect::<Vec<_>>().len() as u64
}

fn main() {
    let input = read_to_string("input6.txt").unwrap();
    let res1 = part_1(input.as_str());
    let res2 = part_2(input.as_str());
    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2, Race};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn parse_test() {
        assert_eq!(parse(INPUT), (
            vec![Race { time: 7, distance: 9 }, Race { time: 15, distance: 40 }, Race { time: 30, distance: 200 }],
            Race { time: 71530, distance: 940200 }
        )
        );
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 288);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 71503);
    }
}