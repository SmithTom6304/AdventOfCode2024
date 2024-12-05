use std::fs;
use day03::InstructionSet;

#[test]
fn example_part1() {
    let value = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let instruction_set = InstructionSet::from(value);
    assert_eq!(161, instruction_set.result())
}

#[test]
fn example_part2() {

}

#[test]
fn result_part1() {
    let file_text = fs::read_to_string("./input").unwrap();
    let instruction_set = InstructionSet::from(file_text.as_str());
    assert_eq!(161, instruction_set.result())
}

#[test]
fn result_part2() {

}
