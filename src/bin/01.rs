advent_of_code::solution!(1);

use std::collections::{ BinaryHeap, HashMap };
use std::collections::hash_map::Entry;
use std::cmp::Reverse;

type MinHeap = BinaryHeap::<Reverse<u32>>;


fn min_heapify( input: &str ) -> ( MinHeap, MinHeap ) {

    let count = input.lines().count();
    let mut first_heap = MinHeap::with_capacity( count );
    let mut second_heap = MinHeap::with_capacity( count );
    input.lines().for_each( |line| {
        let ( first, second ) = line.split_once("   ").unwrap();
        first_heap.push( Reverse(first.parse::<u32>().unwrap() ) );
        second_heap.push( Reverse(second.parse::<u32>().unwrap() ) );
        }
    );
    (first_heap, second_heap )
}



pub fn part_one(input: &str) -> Option<u32> {
    let ( mut first_heap, mut second_heap ) = min_heapify( input );
    let mut sum = 0;
    while first_heap.peek().is_some() {
        sum += u32::abs_diff( first_heap.pop().unwrap().0, second_heap.pop().unwrap().0 );
    }
    Some( sum )
}


fn freqify( input: &str ) -> ( HashMap<u32,u32>, Vec<u32> ) {
    let count = input.lines().count();
    input.lines()
        .fold( (HashMap::<u32,u32>::with_capacity(count), Vec::<u32>::with_capacity(count)), | (mut map, mut vec), line| {
            let ( first, second ) = line.split_once("   ").unwrap();
            vec.push( first.parse::<u32>().unwrap() );
            map.entry(second.parse::<u32>().unwrap() )
                .and_modify( |freq| { *freq+=1; } )
                .or_insert( 1 );
            ( map, vec )
        }
    )
}



pub fn part_two(input: &str) -> Option<u32> {
    let ( mut map, vec ) = freqify( input );
    Some( vec.into_iter().fold( 0u32, | mut acc, val | {
        acc += val*( match map.entry( val ) {
            Entry::Vacant( _ ) => 0u32,
            Entry::Occupied( entry ) => *entry.get()
        });
        acc
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
