use crate::instruction::Instruction;

pub struct InstructionSet {
    instructions: Vec<Instruction>,
}

impl InstructionSet {
    pub fn result(&self) -> u32 {
        self.instructions.iter().map(Instruction::result).sum()
    }

    pub fn from_str_with_filter<F>(value: &str, filter: F) -> InstructionSet
    where
        F: Fn(&str) -> Vec<&str>
    {
        let values = filter(value);
        let instructions = values.into_iter().map(Instruction::from).collect();
        InstructionSet { instructions }
    }
}