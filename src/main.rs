use std::{env::args, panic, time::Instant};

mod lib;
use lib::count_bits;

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

fn day_three_b() -> usize {
    let mut o2_gen = include_str!("../inputs/day3.txt")
        .lines()
        .collect::<Vec<&str>>();
    let mut co2_scrub = o2_gen.clone();

    let width = o2_gen[0].len();
    for i in 0..width {
        let o2_crit = count_bits(&o2_gen, i);
        let co2_crit = count_bits(&co2_scrub, i);

        let (o2_count, co2_count) = (o2_gen.len(), co2_scrub.len());
        if o2_count > 1 {
            o2_gen = o2_gen
                .into_iter()
                .filter(|&line| match o2_crit {
                    1 | 0 => line.chars().nth(i) == Some('1'), //more ones or equal -> keep ones
                    -1 => line.chars().nth(i) == Some('0'),    //more zeros -> keep zeros
                    _ => panic!("Oops"),
                })
                .collect();
        }
        if co2_count > 1 {
            co2_scrub = co2_scrub
                .into_iter()
                .filter(|&line| {
                    match co2_crit {
                        1 | 0 => line.chars().nth(i) == Some('0'), //more ones or equal -> keep zeros
                        -1 => line.chars().nth(i) == Some('1'),    //more zeros -> keep ones
                        _ => panic!("Oops"),
                    }
                })
                .collect();
        }
    }
    let o2_gen = usize::from_str_radix(o2_gen[0], 2).unwrap();
    let co2_scrub = usize::from_str_radix(co2_scrub[0], 2).unwrap();

    o2_gen * co2_scrub
}

fn day_four() -> String {
    String::from("TODO")
}

fn day_four_b() -> String {
    String::from("TODO")
}

fn main() {
    let day = args()
        .nth(1)
        .expect("please specify day to run, i.e. day one -> \"--1\"");
    let now = Instant::now();

    match day.as_str() {
        "--1" => println!("Day 1: a->{} & b->{}", day_one(), day_one_b()),
        "--2" => println!("Day 2: a->{} & b->{}", day_two(), day_two_b()),
        "--3" => println!("Day 3: a->{} & b->{}, ", day_three(), day_three_b()),
        "--4" => println!("Day 4: a->{} & b->{}", day_four(), day_four_b()),

        _ => eprintln!("please specify day to run, i.e. day one -> \"--1\""),
    };

    println!("finished in {} µs", now.elapsed().as_micros());
}
