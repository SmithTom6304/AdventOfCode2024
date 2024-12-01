#[derive(Debug)]
pub struct Pair {
    pub left: u32,
    pub right: u32,
}

impl TryFrom<&str> for Pair {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let values: Vec<&str> = value.split_whitespace().collect();
        let left_str: &str = values.get(0).unwrap();
        let right_str: &str = values.get(1).unwrap();
        let left: u32 = left_str.parse().unwrap();
        let right: u32 = right_str.parse().unwrap();

        Ok(Pair {left, right})
    }
}

impl From<(u32, u32)> for Pair {
    fn from(value: (u32, u32)) -> Self {
        Pair {left: value.0, right: value.1}
    }
}

impl Pair {
    pub fn distance(&self) -> u32 {
        self.left.abs_diff(self.right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_created_from_string() {
        let pair = Pair::try_from("3   4").unwrap();
        assert_eq!(3, pair.left);
        assert_eq!(4, pair.right);
    }

    #[test]
    fn provides_a_distance_between_left_and_right()
    { 
        let pair = Pair::try_from("3   4").unwrap();
        assert_eq!(1, pair.distance());
    }
}
