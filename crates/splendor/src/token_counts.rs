use crate::token::Token;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TokenCounts([u8; 6]);

impl TokenCounts {
    pub fn get(&self, token: Token) -> u8 {
        self.0[token.index()]
    }

    pub fn set(&mut self, token: Token, value: u8) {
        self.0[token.index()] = value;
    }

    pub fn new(counts: [u8; 6]) -> Self {
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
        let counts = TokenCounts::default();
        assert_eq!(counts.total(), 0);
        for token in Token::ALL {
            assert_eq!(counts.get(token), 0);
        }
    }

    #[test]
    fn set_get_roundtrips() {
        let mut counts = TokenCounts::default();
        let values = [1u8, 2, 3, 4, 5, 6];
        for (value, token) in values.into_iter().zip(Token::ALL) {
            counts.set(token, value);
        }
        for (value, token) in values.into_iter().zip(Token::ALL) {
            assert_eq!(counts.get(token), value);
        }
    }

    #[test]
    fn total_sums_all_counts() {
        assert_eq!(TokenCounts::new([1, 2, 3, 4, 5, 6]).total(), 21);
        assert_eq!(TokenCounts::new([255; 6]).total(), 1530);
    }

    #[test]
    fn is_six_bytes() {
        assert_eq!(size_of::<TokenCounts>(), 6);
    }
}
