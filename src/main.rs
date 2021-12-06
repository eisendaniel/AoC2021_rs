use std::time::Instant;

fn day_one() -> usize {
    include_str!("../inputs/dayone.txt")
        .lines()
        .map(|reading| reading.parse().unwrap())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|m| m[0] < m[1])
        .count()
}

fn day_one_b() -> usize {
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

fn day_two_b() -> i32 {
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

fn day_three() -> usize {
    let input = include_str!("../inputs/day3.txt");
    let count = input.lines().count();
    let width = input.lines().last().unwrap().chars().count();

    let gamma_str = input
        .lines()
        .fold(vec![0u32; width as usize], |counts, line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .zip(counts.into_iter())
                .map(|(a, b)| a + b)
                .collect()
        })
        .into_iter()
        .map(|ones| (if ones > (count as u32 / 2) { '1' } else { '0' }))
        .collect::<String>();

    let gamma = usize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = !gamma & ((1 << width) - 1);
    gamma * epsilon
}

fn count_ones(input: &[&str], index: usize) -> usize {
    input.iter().fold(0usize, |n, line| {
        if line.chars().nth(index) == Some('1') {
            n + 1
        } else {
            n
        }
    })
}

fn day_three_b() -> usize {
    let mut input = include_str!("../inputs/day3_ex.txt")
        .lines()
        .collect::<Vec<&str>>();
    let width = input[0].len();

    println!("{:?}", input);
    for i in 0..width {
        let ones = count_ones(&input, i);
        let count = input.len();
        input = input
            .into_iter()
            .filter(|&line| {
                if ones >= count / 2 {
                    line.chars().nth(i) == Some('1')
                } else {
                    line.chars().nth(i) == Some('0')
                }
            })
            .collect();
        println!("{:?}", input);
        if input.len() == 1 {
            break;
        }
    }
    println!("{:?}", input);

    0
}

fn day_four() -> String {
    String::from("TODO")
}

fn day_four_b() -> String {
    String::from("TODO")
}

fn main() {
    let now = Instant::now();

    // println!("Day 1: {} & {}", day_one(), day_one_b());
    // println!("Day 2: {} & {}", day_two(), day_two_b());
    println!("Day 3: {} & {}, ", day_three(), day_three_b());
    // println!("Day 4: {}", day_four(), day_four_b());

    println!("finished in {} µs", now.elapsed().as_micros());
}
