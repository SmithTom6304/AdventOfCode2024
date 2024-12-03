use std::fs;
use day02::Report;

#[test]
fn example_part1() {
    let reports = read_reports_from_file("./test_input");
    assert_eq!(2, reports.into_iter().filter(Report::is_safe).collect::<Vec<_>>().len())
}

#[test]
fn example_part2() {

    let reports = read_reports_from_file("./test_input");
    assert_eq!(4, reports.into_iter().filter(Report::is_safe_with_dampeners).collect::<Vec<_>>().len())
}

#[test]
fn result_part1() {
    let reports = read_reports_from_file("./input");
    assert_eq!(606, reports.into_iter().filter(Report::is_safe).collect::<Vec<_>>().len())
}

#[test]
fn result_part2() {
    let reports = read_reports_from_file("./input");
    assert_eq!(2, reports.into_iter().filter(Report::is_safe_with_dampeners).collect::<Vec<_>>().len())
}

fn read_reports_from_file(path: &str) -> Vec<Report> {
    let binding = fs::read_to_string(path).unwrap();
    let lines = binding.lines();
    lines.map(Report::try_from).map(Result::unwrap).collect()
}
