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
