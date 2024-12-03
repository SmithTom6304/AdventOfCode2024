use crate::Direction;
use crate::Level;

pub struct Report {
    pub levels: Vec<Level>,
}

impl Report {
    pub fn is_safe(&self) -> bool {
        let direction = self.levels[0].determine_direction(&self.levels[1]);

        if direction == Direction::Still {
            return false;
        }

        Self::levels_are_safe(&self.levels, direction)
    }

    fn levels_are_safe(levels: &[Level], direction: Direction) -> bool {
        for i in 1..levels.len() {
            if !levels[i].is_safe(&levels[i-1], direction == Direction::Up) {
                return false;
            }
        }

        true
    }

    pub fn is_safe_with_dampeners(&self) -> bool {
        // Not exactly clever...
        for level_to_skip in 0..self.levels.len() {
            let mut levels = vec![];
            for level in 0 .. self.levels.len() {
                if level == level_to_skip {
                    continue;
                }
                levels.push(self.levels[level]);
            }
            let direction = levels[0].determine_direction(&levels[1]);
            if Self::levels_are_safe(&levels, direction) {
                return true;
            }
        }

        false
    }

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