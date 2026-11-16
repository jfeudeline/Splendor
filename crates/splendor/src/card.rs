use crate::{gem::Gem, gem_counts::GemCounts, level::Level};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub level: Level,
    pub bonus: Gem,
    pub points: u8,
    pub cost: GemCounts,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_stores_its_fields() {
        let card = Card {
            level: Level::One,
            bonus: Gem::Ruby,
            points: 0,
            cost: GemCounts::new([0, 1, 0, 2, 0]),
        };

        assert_eq!(card.level, Level::One);
        assert_eq!(card.bonus, Gem::Ruby);
        assert_eq!(card.points, 0);
        assert_eq!(card.cost.get(Gem::Sapphire), 1);
        assert_eq!(card.cost.get(Gem::Ruby), 2);
    }

    #[test]
    fn card_with_points_stores_them() {
        let card = Card {
            level: Level::Three,
            bonus: Gem::Onyx,
            points: 4,
            cost: GemCounts::new([0, 0, 0, 0, 7]),
        };

        assert_eq!(card.level, Level::Three);
        assert_eq!(card.points, 4);
        assert_eq!(card.cost.get(Gem::Onyx), 7);
    }

    #[test]
    fn card_is_eight_bytes() {
        assert_eq!(std::mem::size_of::<Card>(), 8);
    }
}
