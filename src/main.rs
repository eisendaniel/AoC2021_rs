fn day_one() -> usize {
    include_str!("../inputs/dayone.txt")
        .lines()
        .map(|reading| reading.parse().unwrap())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|m| m[0] < m[1])
        .count()
}

fn day_one_two() -> usize {
    include_str!("../inputs/dayone.txt")
        .lines()
        .map(|reading| reading.parse().unwrap())
        .collect::<Vec<usize>>()
        .windows(4)
        .filter(|m| m[0] < m[3])
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

fn day_two_two() -> i32 {
    let input: Vec<&str> = include_str!("../inputs/daytwo.txt").lines().collect();

    let mut aim = 0;
    let mut depth = 0;
    let mut h_pos = 0;

    for command in input {
        let mut split = command.split_whitespace();
        match split.next() {
            Some("forward") => {
                let x = split.next().unwrap().parse::<i32>().unwrap();
                h_pos += x;
                depth += aim * x;
            }
            Some("down") => aim += split.next().unwrap().parse::<i32>().unwrap(),
            Some("up") => aim -= split.next().unwrap().parse::<i32>().unwrap(),
            _ => panic!("UNKNOWN TOKEN"),
        }
    }

    depth * h_pos
}

fn day_three() -> String {
    String::from("TODO")
}

fn day_four() -> String {
    String::from("TODO")
}

fn day_five() -> String {
    String::from("TODO")
}

fn day_six() -> String {
    String::from("TODO")
}

fn main() {
    println!("Day 1: {} & {}", day_one(), day_one_two());
    println!("Day 2: {} & {}", day_two(), day_two_two());
    println!("Day 3: {}", day_three());
    println!("Day 4: {}", day_four());
    println!("Day 5: {}", day_five());
    println!("Day 6: {}", day_six());
}
