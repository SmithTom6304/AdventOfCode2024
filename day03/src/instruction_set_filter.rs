use regex::Regex;

pub struct InstructionSetFilter {}

impl InstructionSetFilter {
    pub fn filter(value: &str) -> Vec<&str> {
        let regex = Regex::new(r"(mul\()\d{1,3},\d{1,3}\)").unwrap();
        let res = regex.find_iter(value).map(|mat| mat.as_str()).collect::<Vec<&str>>();
        res
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::instruction_set_filter::InstructionSetFilter;

    #[rstest]
    #[case("ABmul(44,46)XY", vec!["mul(44,46)"])]
    #[case("ABmul(44,46)XYmul(1,2)", vec!["mul(44,46)", "mul(1,2)"])]
    fn is_created_from_valid_string(#[case] string: &str, #[case] expected: Vec<&str>) {
        assert_eq!(expected, InstructionSetFilter::filter(string));
    }
}