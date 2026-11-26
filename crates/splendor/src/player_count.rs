use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InvalidPlayerCount(u8);

impl fmt::Display for InvalidPlayerCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid player count: {} (expected 2, 3 or 4)", self.0)
    }
}

impl std::error::Error for InvalidPlayerCount {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayerCount {
    Two,
    Three,
    Four,
}

impl TryFrom<u8> for PlayerCount {
    type Error = InvalidPlayerCount;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            _ => Err(InvalidPlayerCount(value)),
        }
    }
}

impl PlayerCount {
    pub const ALL: [PlayerCount; 3] = [PlayerCount::Two, PlayerCount::Three, PlayerCount::Four];

    pub fn count(self) -> u8 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_contains_each_possible_count() {
        assert_eq!(PlayerCount::ALL.len(), 3);
        for (expected, players) in PlayerCount::ALL.into_iter().enumerate() {
            assert_eq!(players.count() as usize, expected + 2);
        }
    }

    #[test]
    fn try_from_returns_count_or_error() {
        assert_eq!(PlayerCount::try_from(0), Err(InvalidPlayerCount(0)));
        assert_eq!(PlayerCount::try_from(1), Err(InvalidPlayerCount(1)));
        assert_eq!(PlayerCount::try_from(2), Ok(PlayerCount::Two));
        assert_eq!(PlayerCount::try_from(3), Ok(PlayerCount::Three));
        assert_eq!(PlayerCount::try_from(4), Ok(PlayerCount::Four));
        assert_eq!(PlayerCount::try_from(5), Err(InvalidPlayerCount(5)));
        assert_eq!(PlayerCount::try_from(10), Err(InvalidPlayerCount(10)));
        assert_eq!(
            PlayerCount::try_from(u8::MAX),
            Err(InvalidPlayerCount(u8::MAX))
        );
    }

    #[test]
    fn count_roundtrips() {
        for players in PlayerCount::ALL {
            assert_eq!(PlayerCount::try_from(players.count()), Ok(players));
        }
    }

    #[test]
    fn displays_error_message() {
        assert!(InvalidPlayerCount(0).to_string().contains("0 (expected"));
    }

    #[test]
    fn player_count_is_one_byte() {
        assert_eq!(std::mem::size_of::<PlayerCount>(), 1);
    }
}
