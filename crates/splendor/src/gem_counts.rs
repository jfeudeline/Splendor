use crate::gem::Gem;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GemCounts([u8; 5]);

impl GemCounts {
    pub fn get(&self, gem: Gem) -> u8 {
        self.0[gem.index()]
    }

    pub fn set(&mut self, gem: Gem, value: u8) {
        self.0[gem.index()] = value;
    }

    pub fn new(counts: [u8; 5]) -> Self {
        Self(counts)
    }

    pub fn total(&self) -> u16 {
        let mut total: u16 = 0;
        for count in self.0 {
            total += u16::from(count);
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_zero() {
        let counts = GemCounts::default();
        assert_eq!(counts.total(), 0);
        for gem in Gem::ALL {
            assert_eq!(counts.get(gem), 0);
        }
    }

    #[test]
    fn set_get_roundtrips() {
        let mut counts = GemCounts::default();
        let values = [1u8, 2, 3, 4, 5];
        for (value, gem) in values.into_iter().zip(Gem::ALL) {
            counts.set(gem, value);
        }
        for (value, gem) in values.into_iter().zip(Gem::ALL) {
            assert_eq!(counts.get(gem), value);
        }
    }

    #[test]
    fn total_sums_all_counts() {
        assert_eq!(GemCounts::new([1, 2, 3, 4, 5]).total(), 15);
        assert_eq!(GemCounts::new([255; 5]).total(), 1275);
    }

    #[test]
    fn is_five_bytes() {
        assert_eq!(size_of::<GemCounts>(), 5);
    }
}
