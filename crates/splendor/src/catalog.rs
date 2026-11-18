use crate::{card::Card, noble::Noble};
use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
pub struct Catalog {
    pub cards: Vec<Card>,
    pub nobles: Vec<Noble>,
}

static CATALOG: LazyLock<Catalog> = LazyLock::new(|| {
    ron::from_str(include_str!("../data/catalog.ron")).expect("invalid catalog data")
});

pub fn catalog() -> &'static Catalog {
    &CATALOG
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CARD_COUNT;
    use crate::gem::Gem;
    use crate::level::Level;
    use crate::noble::NOBLE_COUNT;

    #[test]
    fn catalog_has_expected_counts() {
        let catalog = catalog();

        assert_eq!(catalog.cards.len(), CARD_COUNT);
        assert_eq!(catalog.nobles.len(), NOBLE_COUNT);

        let counts = Level::ALL.map(|level| {
            catalog
                .cards
                .iter()
                .filter(|card| card.level == level)
                .count()
        });

        assert_eq!(counts, [40, 30, 20]);
    }

    #[test]
    fn cards_are_grouped_by_level() {
        let cards = &catalog().cards;

        assert!(cards[..40].iter().all(|card| card.level == Level::One));
        assert!(cards[40..70].iter().all(|card| card.level == Level::Two));
        assert!(cards[70..].iter().all(|card| card.level == Level::Three));
    }

    #[test]
    fn card_values_are_in_valid_ranges() {
        for (index, card) in catalog().cards.iter().enumerate() {
            assert!(card.points <= 5, "card {index} has too many points");

            for gem in Gem::ALL {
                assert!(
                    card.cost.get(gem) <= 7,
                    "card {index} costs too many {gem:?}"
                );
            }
        }
    }

    #[test]
    fn nobles_are_valid() {
        for (index, noble) in catalog().nobles.iter().enumerate() {
            assert_eq!(noble.points, 3, "noble {index} must give 3 points");

            let total = noble.requirements.total();
            assert!(
                total == 8 || total == 9,
                "noble {index} has an invalid requirement total: {total}"
            );

            for gem in Gem::ALL {
                let required = noble.requirements.get(gem);
                assert!(required <= 4, "noble {index} requires too many {gem:?}");
            }
        }
    }

    #[test]
    fn catalog_is_initialized_once() {
        assert!(std::ptr::eq(catalog(), catalog()));
    }
}
