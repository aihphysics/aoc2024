advent_of_code::solution!(3);

use regex::Captures;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let reg = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    Some(
        reg.captures_iter(input)
            .map(|c: Captures| c.extract::<2>())
            .map(|(_, vals)| vals)
            .fold(0u32, |acc, vals| acc + vals[0].parse::<u32>().unwrap() * vals[1].parse::<u32>().unwrap())
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let reg = Regex::new(r"(do\(\)|don't\(\)).*mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    Some( reg.captures_iter(input)
            .map(|c: Captures| c.extract::<3>())
            .map(|(_, vals)| vals)
            .fold(0u32, |acc, vals| {
                println!( "{:?}", vals );
                if vals[0] == "do()" {
                    acc + vals[1].parse::<u32>().unwrap() * vals[2].parse::<u32>().unwrap()
                } else {
                    acc
                }
            }
            )
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
