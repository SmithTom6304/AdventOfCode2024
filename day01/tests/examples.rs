use std::fs;

use day01::ordered_list::OrderedList;
use day01::unordered_list::UnorderedList;

#[test]
fn example_part1() {
        assert_eq!(11, find_distance_from_file("./test_input"))
}

#[test]
fn example_part2() {
        assert_eq!(31, find_similarity_from_file("./test_input"))
}

#[test]
fn result_part1() {
        assert_eq!(2970687, find_distance_from_file("./input"))
}

#[test]
fn result_part2() {
        assert_eq!(23963899, find_similarity_from_file("./input"))
}

fn find_distance_from_file(path: &str) -> u32 {
    let binding = fs::read_to_string(path).unwrap();
    let lines = binding.lines();
    let unordered_list = UnorderedList::try_from(lines).unwrap();
    let ordered_list: OrderedList = unordered_list.into();
    let total_distance: u32 = ordered_list.pairs.into_iter().map(|pair| pair.distance()).sum();
    total_distance
}

fn find_similarity_from_file(path: &str) -> u32 {
    let binding = fs::read_to_string(path).unwrap();
    let lines = binding.lines();
    let unordered_list = UnorderedList::try_from(lines).unwrap();
    unordered_list.similarity()
}
