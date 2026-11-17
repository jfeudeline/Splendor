use crate::{gem::Gem, gem_counts::GemCounts, level::Level};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub level: Level,
    pub bonus: Gem,
    pub points: u8,
    pub cost: GemCounts,
}

pub const CARD_COUNT: usize = 90;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CardId(u8);

impl CardId {
    pub fn new(value: u8) -> Option<Self> {
        if usize::from(value) < CARD_COUNT {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        u8::try_from(index).ok().and_then(Self::new)
    }

    pub fn index(self) -> usize {
        usize::from(self.0)
    }

    pub fn get(self) -> u8 {
        self.0
    }
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

    #[test]
    fn card_id_new_is_valid() {
        assert_eq!(CardId::new(0), Some(CardId(0)));
        assert_eq!(CardId::new(89), Some(CardId(89)));
        assert_eq!(CardId::new(90), None);
        assert_eq!(CardId::new(255), None);
    }

    #[test]
    fn from_index_rejects_out_of_range() {
        assert_eq!(CardId::from_index(90), None);
        assert_eq!(CardId::from_index(usize::MAX), None);
        assert_eq!(CardId::from_index(CARD_COUNT), None);
    }

    #[test]
    fn index_roundtrips() {
        for index in [0, 1, 42, 89] {
            let id = CardId::from_index(index).unwrap();
            assert_eq!(id.index(), index);
            assert_eq!(CardId::from_index(id.index()), Some(id));
        }
    }

    #[test]
    fn card_id_is_one_byte() {
        assert_eq!(std::mem::size_of::<CardId>(), 1);
    }
}
