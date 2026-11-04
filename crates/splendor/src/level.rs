/// The level of a development card.
///
/// Levels are numbered 1 to 3 in the rules. [`Level::value`] keeps that
/// numbering, while [`Level::index`] maps the levels to the 0-based positions
/// used by arrays.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    /// Level 1 development card.
    One = 1,
    /// Level 2 development card.
    Two = 2,
    /// Level 3 development card.
    Three = 3,
}

impl Level {
    /// All levels in ascending order.
    pub const ALL: [Level; 3] = [Level::One, Level::Two, Level::Three];

    /// The level as used by the rules and card data: 1, 2 or 3.
    pub const fn value(self) -> u8 {
        match self {
            Level::One => 1,
            Level::Two => 2,
            Level::Three => 3,
        }
    }

    /// The 0-based position used to index arrays: 0, 1 or 2.
    pub const fn index(self) -> usize {
        match self {
            Level::One => 0,
            Level::Two => 1,
            Level::Three => 2,
        }
    }

    /// Returns the level at a 0-based array position.
    pub fn from_index(index: usize) -> Option<Level> {
        Level::ALL.get(index).copied()
    }

    /// Returns the level for a rules value (1, 2 or 3).
    pub fn from_value(value: u8) -> Option<Level> {
        match value {
            1 => Some(Level::One),
            2 => Some(Level::Two),
            3 => Some(Level::Three),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_levels_in_order() {
        assert_eq!(Level::ALL.len(), 3);
        assert!(Level::One < Level::Two);
        assert!(Level::Two < Level::Three);
    }

    #[test]
    fn value_is_one_based() {
        assert_eq!(Level::One.value(), 1);
        assert_eq!(Level::Two.value(), 2);
        assert_eq!(Level::Three.value(), 3);
    }

    #[test]
    fn index_is_zero_based() {
        assert_eq!(Level::One.index(), 0);
        assert_eq!(Level::Two.index(), 1);
        assert_eq!(Level::Three.index(), 2);
    }

    #[test]
    fn index_is_a_valid_array_index() {
        let decks = [10u8, 20, 30];
        assert_eq!(decks[Level::Two.index()], 20);
    }

    #[test]
    fn index_roundtrips() {
        for level in Level::ALL {
            assert_eq!(Level::from_index(level.index()), Some(level));
        }
        assert_eq!(Level::from_index(3), None);
    }

    #[test]
    fn value_roundtrips() {
        for level in Level::ALL {
            assert_eq!(Level::from_value(level.value()), Some(level));
        }
        assert_eq!(Level::from_value(0), None);
        assert_eq!(Level::from_value(4), None);
    }

    #[test]
    fn discriminants_match_value() {
        for level in Level::ALL {
            assert_eq!(level as u8, level.value());
        }
    }

    #[test]
    fn level_is_one_byte() {
        assert_eq!(std::mem::size_of::<Level>(), 1);
    }
}
