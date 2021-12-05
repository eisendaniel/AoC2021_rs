fn day_one() -> usize {
    include_str!("../inputs/dayone.txt")
        .lines()
        .map(|reading| reading.parse().unwrap())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|m| m[0] < m[1])
        .count()
}

fn day_two() -> u32 {
    let input: Vec<&str> = include_str!("../inputs/daytwo.txt").lines().collect();

    let mut depth: u32 = 0;
    let mut h_pos: u32 = 0;

    for command in input {
        let mut split = command.split_whitespace();
        match split.next() {
            Some("forward") => h_pos += split.next().unwrap().parse::<u32>().unwrap(),
            Some("down") => depth += split.next().unwrap().parse::<u32>().unwrap(),
            Some("up") => depth -= split.next().unwrap().parse::<u32>().unwrap(),
            _ => panic!("UNKNOWN TOKEN"),
        }
    }

    depth * h_pos
}

fn main() {
    // println!("Day 1: {}", day_one());
    println!("Day 2: {}", day_two());
}
