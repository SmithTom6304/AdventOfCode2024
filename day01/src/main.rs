use std::fs;
use day01::{UnorderedList, OrderedList};

fn main() {
    let binding = fs::read_to_string("./input").unwrap();
    let lines = binding.lines();
    let unordered_list = UnorderedList::try_from(lines).unwrap();
    let ordered_list: OrderedList = unordered_list.into();
    let total_distance: u32 = ordered_list.pairs.into_iter().map(|pair| pair.distance()).sum();
    println!("Total distance: {}", total_distance);
}
