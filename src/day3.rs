use std::{collections::HashMap, ops::Add};

use crate::helpers::helpers::load_input;

pub fn run() {
    let input = load_input("day3".to_string());
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}


fn part1(input: &String) -> i32 {
    let mut data = [(0,0);12];

    for l in input.lines() {
        for (i, c) in l.chars().rev().enumerate() {
            let (zero, one) = data[i];
            match c {
                '0' => {data[i] = (zero + 1, one)},
                '1' => {data[i] = (zero, one + 1)},
                s => println!("Unhandled case! {}", s)
            }
        }
    }
    //println!("{:?}", data);
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, (zero, one)) in data.iter().enumerate() {
        if zero < one {
            // 1 is more common
            gamma += 1 * (2_i32).pow(i as u32)
        } else if zero > one {
            // 0 is more common
            epsilon += 1 * (2_i32).pow(i as u32)
        }
    }

    return gamma * epsilon
}

fn part2(input: &String) -> i32 {
    let mut data = [(0,0);12];
    let mut numbers = HashMap::<String, i32>::new();


    for l in input.lines() {
        if l == "" {continue;}
        numbers.insert(l.to_owned(), 1);
        for (i, c) in l.chars().enumerate() {
            let (zero, one) = data[i];
            match c {
                '0' => {data[i] = (zero + 1, one)},
                '1' => {data[i] = (zero, one + 1)},
                s => println!("Unhandled case! {}", s)
            }
        }
    }
    println!("{:?}", numbers.keys());


    println!("{:?}", data);
    let mut oxygen_str = "".to_owned();
    let mut new_numbers = numbers.clone();
    println!("{:?}", new_numbers.keys());

    for (zero, one) in data {
        if zero <= one {
            // 1 is more common
            oxygen_str += "1";
        } else {
            // 0 is more common
            oxygen_str += "0";
        }
        new_numbers = numbers_left(oxygen_str.as_str(), new_numbers);
        println!("{:?}", new_numbers.keys());
        if new_numbers.keys().len() == 1 {
            oxygen_str = new_numbers.into_keys().collect();
            break;
        }
    }
    println!("{}", oxygen_str);

    return 0;
}

fn numbers_left(part: &str, numbers: HashMap<String, i32>) -> HashMap<String, i32> {
    let mut new = HashMap::<String, i32>::new();
    let part_length = part.len();
    if part_length == 12 {
        new.insert(part.to_owned(), 1);
        return new;
    }

    let b = i64::from_str_radix((0..12-part_length).map(|_| '1').collect::<String>().as_str(), 2).unwrap();
    println!("part: {}, max: {}", part, b);
    for i in 0..b+1 {
        let bin_str = format!("{:b}", i);
        let padding: String = (part_length..12-bin_str.len()).map(|_| '0').collect();
        let key = format!("{}{}{}", part, padding, bin_str);
        //println!("Key gen: {}", key);
        if numbers.contains_key(&key) {
            new.insert(key, 1);
        }
    }

    return new;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    00100\n
    11110\n
    10110\n
    10111\n
    10101\n
    01111\n
    00111\n
    11100\n
    10000\n
    11001\n
    00010\n
    01010";

    const INPUT2: &str = "
    000000000100\n
    000000011110\n
    000000010110\n
    000000010111\n
    000000010101\n
    000000001111\n
    000000000111\n
    000000011100\n
    000000010000\n
    000000011001\n
    000000000010\n
    000000001010";

    #[test]
    fn test_part1() {
        assert_eq!(198, part1(&INPUT.replace(" ", "").to_string()))
    }
    #[test]
    fn test_part2() {
        assert_eq!(5, part2(&INPUT2.replace(" ", "").to_string()))
    }
    
}