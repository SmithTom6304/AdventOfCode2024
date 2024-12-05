#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Instruction {
    pub left: u16,
    pub right: u16,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        // value has been filtered by regex, so this should be guaranteed
        let open_br_i = value.find('(').unwrap();
        let close_br_i = value.find(')').unwrap();
        let values = &value[open_br_i + 1..close_br_i];
        let values: Vec<&str> = values.split(',').collect();
        let left = values[0].parse().unwrap();
        let right = values[1].parse().unwrap();
        Instruction { left, right }
    }
}

impl Instruction {
    pub fn result(&self) -> u32 {
        self.left as u32 * self.right as u32
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::instruction::Instruction;

    #[rstest]
    #[case("mul(44,46)", Instruction { left: 44, right: 46 })]
    #[case("mul(0,999)", Instruction { left: 0, right: 999 })]
    fn is_created_from_valid_string(#[case] string: &str, #[case] expected: Instruction) {
        assert_eq!(expected, Instruction::from(string));
    }
}