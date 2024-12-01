use crate::Pair;
use crate::UnorderedList;

pub struct OrderedList {
    pub pairs: Vec<Pair>
}

impl From<UnorderedList> for OrderedList {
    fn from(value: UnorderedList) -> Self {
        let mut sorted_lefts = value.pairs.iter().map(|pair| pair.left).collect::<Vec<u32>>();
        let mut sorted_rights = value.pairs.iter().map(|pair| pair.right).collect::<Vec<u32>>();

        sorted_lefts.sort();
        sorted_rights.sort();


        let pairs = sorted_lefts.into_iter().zip(sorted_rights).map(|pair| Pair::from((pair.0, pair.1))).collect();
        OrderedList {pairs}
    }
}
