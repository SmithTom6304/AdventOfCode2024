use std::str::Lines;
use crate::Pair;

pub struct UnorderedList {
    pub pairs: Vec<Pair>
}

impl TryFrom<Lines<'_>> for UnorderedList {
    type Error = &'static str;

    fn try_from(value: Lines) -> Result<Self, Self::Error> {
        let pairs = value.map(|line| Pair::try_from(line).unwrap()).collect();

        Ok(UnorderedList{ pairs })
    }
}

impl UnorderedList {
    pub fn similarity(&self) -> u32 {
        let left_values = self.pairs.iter().map(|pair| pair.left);
        let right_values = self.pairs.iter().map(|pair| pair.right);
        left_values.map(|left| left * right_values.clone().filter(|right| &left == right).collect::<Vec<u32>>().len() as u32).sum()
    }
}
