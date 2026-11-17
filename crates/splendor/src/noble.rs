use crate::gem_counts::GemCounts;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Noble {
    pub points: u8,
    pub requirements: GemCounts,
}

pub const NOBLE_COUNT: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NobleId(u8);

impl NobleId {
    pub fn new(value: u8) -> Option<Self> {
        if usize::from(value) < NOBLE_COUNT {
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
    use crate::gem::Gem;

    #[test]
    fn noble_stores_its_fields() {
        let noble = Noble {
            points: 3,
            requirements: GemCounts::new([0, 3, 0, 3, 3]),
        };

        assert_eq!(noble.points, 3);
        assert_eq!(noble.requirements.get(Gem::Sapphire), 3);
        assert_eq!(noble.requirements.get(Gem::Ruby), 3);
    }

    #[test]
    fn noble_is_six_bytes() {
        assert_eq!(std::mem::size_of::<Noble>(), 6);
    }

    #[test]
    fn noble_id_new_is_valid() {
        assert_eq!(NobleId::new(0), Some(NobleId(0)));
        assert_eq!(NobleId::new(9), Some(NobleId(9)));
        assert_eq!(NobleId::new(10), None);
        assert_eq!(NobleId::new(255), None);
    }

    #[test]
    fn from_index_rejects_out_of_range() {
        assert_eq!(NobleId::from_index(10), None);
        assert_eq!(NobleId::from_index(usize::MAX), None);
        assert_eq!(NobleId::from_index(NOBLE_COUNT), None);
    }

    #[test]
    fn index_roundtrips() {
        for index in [0, 1, 4, 9] {
            let id = NobleId::from_index(index).unwrap();
            assert_eq!(id.index(), index);
            assert_eq!(NobleId::from_index(id.index()), Some(id));
        }
    }

    #[test]
    fn noble_id_is_one_byte() {
        assert_eq!(std::mem::size_of::<NobleId>(), 1);
    }
}
