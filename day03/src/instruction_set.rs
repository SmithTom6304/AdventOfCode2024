use crate::instruction::Instruction;
use crate::instruction_set_filter::InstructionSetFilter;

pub struct InstructionSet {
    instructions: Vec<Instruction>,
}

impl InstructionSet {
    pub fn result(&self) -> u32 {
        self.instructions.iter().map(Instruction::result).sum()
    }
}

impl From<&str> for InstructionSet {
    fn from(value: &str) -> Self {
        let values = InstructionSetFilter::filter(value);
        let instructions = values.into_iter().map(Instruction::from).collect();
        InstructionSet { instructions }
    }
}