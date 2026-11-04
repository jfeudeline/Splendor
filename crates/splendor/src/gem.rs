#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Gem {
    Diamond = 0,
    Sapphire = 1,
    Emerald = 2,
    Ruby = 3,
    Onyx = 4,
}

impl Gem {
    pub const ALL: [Gem; 5] = [
        Gem::Diamond,
        Gem::Sapphire,
        Gem::Emerald,
        Gem::Ruby,
        Gem::Onyx,
    ];

    pub fn index(self) -> usize {
        match self {
            Gem::Diamond => 0,
            Gem::Sapphire => 1,
            Gem::Emerald => 2,
            Gem::Ruby => 3,
            Gem::Onyx => 4,
        }
    }

    pub fn from_index(index: usize) -> Option<Gem> {
        Gem::ALL.get(index).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_contains_each_gem_once() {
        assert_eq!(Gem::ALL.len(), 5);
        for (expected, gem) in Gem::ALL.into_iter().enumerate() {
            assert_eq!(gem.index(), expected);
        }
    }

    #[test]
    fn discriminants_match_index() {
        for gem in Gem::ALL {
            assert_eq!(gem as usize, gem.index());
        }
    }

    #[test]
    fn index_roundtrips() {
        for gem in Gem::ALL {
            assert_eq!(Gem::from_index(gem.index()), Some(gem));
        }
        assert_eq!(Gem::from_index(5), None);
        assert_eq!(Gem::from_index(usize::MAX), None);
    }

    #[test]
    fn gem_is_one_byte() {
        assert_eq!(std::mem::size_of::<Gem>(), 1);
    }
}
