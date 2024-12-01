use std::fs;

use day01::ordered_list::OrderedList;
use day01::unordered_list::UnorderedList;

#[test]
fn can_find_total_distance() {
    let binding = fs::read_to_string("./test_input").unwrap();
    let lines = binding.lines();
    let unordered_list = UnorderedList::try_from(lines).unwrap();
    let ordered_list: OrderedList = unordered_list.into();
    let total_distance: u32 = ordered_list.pairs.into_iter().map(|pair| pair.distance()).sum();
    assert_eq!(11, total_distance)
}
