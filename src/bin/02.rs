advent_of_code::solution!(2);

use std::cmp::Reverse;
use std::error::Error;

fn arranged(levels: &Vec<u32>) -> bool {
    levels.is_sorted() || levels.is_sorted_by(|a, b| a > b)
}


fn safe(a: &u32, b: &u32) -> Result<(), ()> {
    let d = a.abs_diff(*b);
    (d > 0 && d < 4).then_some(()).ok_or(())
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0u32, |acc, line| {
        let levels: Vec<u32> = line
            .split(' ')
            .map(|x: &str| x.parse::<u32>().unwrap())
            .collect();
        if arranged(&levels) {
            match levels.windows(2).try_for_each(|x| safe(&x[0], &x[1])) {
                Ok(_) => acc + 1,
                Err(()) => acc,
            }
        } else {
            acc
        }
    }))
}

fn safe2(a: &u32, b: &u32) -> bool {
    let d = a.abs_diff(*b);
    d > 0 && d < 4
}

//fn failure( levels: &Vec<u32> ) -> Option<u32> {
//
//    levels.windows( 2 ).skip_while( |x| !safe2( &x[0],&x[1]) )
//
//}


pub fn part_two(input: &str) -> Option<u32> {
//    input.lines().for_each( |line| {
//        let levels:Vec<u32> = line.split_by_whitespace()
//            .map(|x: &str| x.parse::<u32>().unwrap())
//            .collect();
//        }
//    );
    None
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
