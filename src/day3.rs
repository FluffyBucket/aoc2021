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
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "00100\n
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

    #[test]
    fn test_part1() {
        assert_eq!(198, part1(&INPUT.replace(" ", "").to_string()))
    }
    #[test]
    fn test_part2() {
        assert_eq!(5, part2(&INPUT.to_string()))
    }
    
}