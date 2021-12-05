use crate::helpers::helpers::load_input;

pub fn run() {
    let input = load_input("day2".to_string());
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}


fn part1(input: &String) -> i32 {
    let mut x = 0;
    let mut y = 0;
    
    for l in input.lines() {
        let code: Vec<&str> = l.split(' ').collect();
        let cmd = code[0];
        let val = code[1].parse::<i32>().unwrap();
        match cmd {
            "forward" => x += val,
            "down" => y += val,
            "up" => y -= val,
            s => println!("Unhandled case: {}", s)
        }
    }

    return x * y
}

fn part2(input: &String) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    
    for l in input.lines() {
        let code: Vec<&str> = l.split(' ').collect();
        let cmd = code[0];
        let val = code[1].parse::<i32>().unwrap();
        match cmd {
            "forward" => {x += val;
                          y += val * aim},
            "down" => {aim += val},
            "up" => {aim -= val},
            s => println!("Unhandled case: {}", s)
        }
    }

    return x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn test_part1() {
        assert_eq!(150, part1(&INPUT.to_string()))
    }
    #[test]
    fn test_part2() {
        assert_eq!(900, part2(&INPUT.to_string()))
    }
    
}