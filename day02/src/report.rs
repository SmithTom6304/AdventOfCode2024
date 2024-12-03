use std::cmp::{Ordering, PartialEq};
use crate::Level;

pub struct Report {
    pub levels: Vec<Level>,
}

impl Report {
    pub fn is_safe(&self) -> bool {
        let direction = self.determine_direction();

        if direction == Direction::Still {
            return false;
        }

        for i in 1..self.levels.len() {
            if !self.levels[i].is_safe(&self.levels[i-1], direction == Direction::Up) {
                return false;
            }
        }

        true
    }

    pub fn is_safe_with_dampeners(&self) -> bool {
        let mut has_been_dampened = false;

        let direction = self.determine_direction();

        if direction == Direction::Still {
            if has_been_dampened {
                return false;
            }
            has_been_dampened = true;
        }

        for i in 1..self.levels.len() {
            if !self.levels[i].is_safe(&self.levels[i-1], direction == Direction::Up) {
                if has_been_dampened {
                    return false;
                }
                has_been_dampened = true;
            }
        }

        true
    }

    fn determine_direction(&self) -> Direction {
        match &self.levels[0].cmp(&self.levels[1]) {
            Ordering::Less => Direction::Up,
            Ordering::Equal => Direction::Still,
            Ordering::Greater => Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Still
}

impl TryFrom<&str> for Report {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let levels = value.split_whitespace();
        let levels = levels
            .map(Level::try_from)
            .map(|result| result.unwrap())
            .collect();

        Ok(Self { levels })
    }
}