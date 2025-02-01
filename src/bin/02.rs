advent_of_code::solution!(2);

use std::cmp::Reverse;
use std::error::Error;

fn arranged(levels: &[u32]) -> bool {
    levels.is_sorted() || levels.is_sorted_by(|a, b| a > b)
}

// error wrapping
fn safe_res(a: &u32, b: &u32) -> Result<(), ()> {
    let d = a.abs_diff(*b);
    (d > 0 && d < 4).then_some(()).ok_or(())
}

fn safe_bool(a: &u32, b: &u32) -> bool {
    let d = a.abs_diff(*b);
    d > 0 && d < 4
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0u32, |acc, line| {
        let levels: Vec<u32> = line
            .split(' ')
            .map(|x: &str| x.parse::<u32>().unwrap())
            .collect();
        if arranged(&levels) {
            match levels.windows(2).try_for_each(|x| safe_res(&x[0], &x[1])) {
                Ok(_) => acc + 1,
                Err(()) => acc,
            }
        } else {
            acc
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().fold(0u32, |acc, line| {
        // get the levels
        let levels: Vec<u32> = line
            .split_whitespace()
            .map(|x: &str| x.parse::<u32>().unwrap())
            .collect();
        match levels.windows(2).try_for_each(|x| safe_res(&x[0], &x[1])) {
            Ok(_) => {
                if arranged(&levels) {
                    acc + 1
                } else {
                    acc
                }
            }
            Err(()) => {
                let val = (0..levels.len()).fold(0u32, |count, removable| {
                    let filtered: Vec<u32> = levels
                        .iter()
                        .enumerate()
                        .filter_map(|(index, res)| {
                            if index == removable {
                                None
                            } else {
                                Some(res.clone())
                            }
                        })
                        .collect();
                    if !arranged(&filtered) {
                        count
                    } else {
                        match filtered.windows(2).try_for_each(|x| safe_res(&x[0], &x[1])) {
                            Ok(_) => {
                                println!("{:?}", filtered);
                                count + 1
                            }
                            Err(_) => count,
                        }
                    }
                });
                if val >= 1u32 {
                    acc + 1
                } else {
                    acc
                }
            }
        }
    }))
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
