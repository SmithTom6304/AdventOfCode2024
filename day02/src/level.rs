use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Level(pub i8);

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Still
}

impl Level {
    pub fn is_safe(&self, previous: &Level, going_up: bool) -> bool {
        match going_up {
            true => previous.0 < self.0 && self.0 <= previous.0 + 3,
            false => previous.0 > self.0 && self.0 >= previous.0 - 3,
        }
    }

    pub fn determine_direction(&self, other: &Level) -> Direction {
        match &self.cmp(other) {
            Ordering::Less => Direction::Up,
            Ordering::Equal => Direction::Still,
            Ordering::Greater => Direction::Down,
        }
    }
}

impl TryFrom<&str> for Level {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let level = value.parse::<i8>().unwrap();
        Ok(Level(level))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::Level;

    #[rstest]
    #[case(Level(0), Level(1), true)]
    #[case(Level(0), Level(3), true)]
    #[case(Level(1), Level(0), false)]
    #[case(Level(3), Level(0), false)]
    fn safe_levels(#[case] previous: Level, #[case] next: Level, #[case] going_up: bool) {
        assert!(next.is_safe(&previous, going_up));
    }

    #[rstest]
    #[case(Level(1), Level(0), true)]
    #[case(Level(1), Level(1), true)]
    #[case(Level(0), Level(4), true)]
    #[case(Level(0), Level(1), false)]
    #[case(Level(4), Level(0), false)]
    #[case(Level(3), Level(3), false)]
    fn unsafe_levels(#[case] previous: Level, #[case] next: Level, #[case] going_up: bool) {
        assert_eq!(false, next.is_safe(&previous, going_up));
    }
}