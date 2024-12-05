use std::fs;
use day03::{InstructionSet, InstructionSetFilter};

#[test]
fn example_part1() {
    let value = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let instruction_set = InstructionSet::from_str_with_filter(value, InstructionSetFilter::filter);
    assert_eq!(161, instruction_set.result())
}

#[test]
fn example_part2() {
    let value = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let instruction_set = InstructionSet::from_str_with_filter(value, InstructionSetFilter::filter_with_toggles);
    assert_eq!(48, instruction_set.result())
}

#[test]
fn result_part1() {
    let file_text = fs::read_to_string("./input").unwrap();
    let instruction_set = InstructionSet::from_str_with_filter(file_text.as_str(), InstructionSetFilter::filter);
    assert_eq!(170807108, instruction_set.result())
}

#[test]
fn result_part2() {
    let file_text = fs::read_to_string("./input").unwrap();
    let instruction_set = InstructionSet::from_str_with_filter(file_text.as_str(), InstructionSetFilter::filter_with_toggles);
    assert_eq!(74838033, instruction_set.result())
}
