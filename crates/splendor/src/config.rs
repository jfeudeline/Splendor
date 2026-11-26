use crate::player_count::PlayerCount;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    pub players: PlayerCount,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configs_with_different_player_counts_differ() {
        let conf1 = Config {
            players: PlayerCount::Two,
        };
        let conf2 = Config {
            players: PlayerCount::Three,
        };
        let conf3 = Config {
            players: PlayerCount::Four,
        };
        assert_ne!(conf1, conf2);
        assert_ne!(conf1, conf3);
        assert_ne!(conf3, conf2);
    }

    #[test]
    fn config_owns_the_right_value() {
        assert_eq!(
            Config {
                players: PlayerCount::Four
            }
            .players
            .count(),
            4
        );
    }

    #[test]
    fn config_is_one_byte() {
        assert_eq!(std::mem::size_of::<Config>(), 1);
    }
}
