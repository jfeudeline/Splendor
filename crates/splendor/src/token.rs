use crate::gem::Gem;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Gem(Gem),
    Gold,
}

impl Token {
    pub const ALL: [Token; 6] = [
        Token::Gem(Gem::Diamond),
        Token::Gem(Gem::Sapphire),
        Token::Gem(Gem::Emerald),
        Token::Gem(Gem::Ruby),
        Token::Gem(Gem::Onyx),
        Token::Gold,
    ];

    pub fn index(self) -> usize {
        match self {
            Token::Gem(gem) => gem.index(),
            Token::Gold => Gem::ALL.len(),
        }
    }

    pub fn from_index(index: usize) -> Option<Token> {
        if index == Gem::ALL.len() {
            Some(Token::Gold)
        } else {
            Gem::from_index(index).map(Token::Gem)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_tokens() {
        assert_eq!(Token::Gem(Gem::Diamond), Token::Gem(Gem::Diamond));
        assert_eq!(Token::Gold, Token::Gold);
        assert_ne!(Token::Gem(Gem::Diamond), Token::Gem(Gem::Emerald));
        assert_ne!(Token::Gem(Gem::Diamond), Token::Gold);
    }

    #[test]
    fn index_maps_gems_and_gold() {
        for gem in Gem::ALL {
            assert_eq!(Token::Gem(gem).index(), gem.index());
        }
        assert_eq!(Token::Gold.index(), 5);
    }

    #[test]
    fn all_contains_each_token_once() {
        assert_eq!(Token::ALL.len(), 6);
        for (expected, token) in Token::ALL.into_iter().enumerate() {
            assert_eq!(token.index(), expected);
        }
    }

    #[test]
    fn index_roundtrips() {
        for token in Token::ALL {
            assert_eq!(Token::from_index(token.index()), Some(token));
        }
        assert_eq!(Token::from_index(6), None);
        assert_eq!(Token::from_index(usize::MAX), None);
    }
}
