use crate::helpers::helpers::load_input;

pub fn run() {
    let input = load_input("day1".to_string());
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}


fn part1(input: &String) -> i32 {
    let numbers = input.lines().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut n_iter = numbers.iter();
    let mut curr = n_iter.next().unwrap();
    let mut count = 0;
    for n in n_iter {
        if n > curr {
            count += 1;
        }
        curr = n;
    }
    return count;
}

fn part2(input: &String) -> i32 {
    let numbers = input.lines().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut n_iter = numbers.iter().as_slice().windows(3);
    let mut curr = n_iter.next().unwrap().iter().fold(0, |sum, x| sum + x);
    let mut count = 0;
    for ns in n_iter {
        let new = ns.iter().fold(0, |sum, x| sum + x);
        if new > curr {
            count += 1;
        }
        curr = new;
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_part1() {
        assert_eq!(7, part1(&INPUT.to_string()))
    }
    #[test]
    fn test_part2() {
        assert_eq!(5, part2(&INPUT.to_string()))
    }
    
}