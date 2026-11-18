use splendor::catalog::catalog;
use splendor::gem::Gem;

fn main() {
    let catalog = catalog();

    println!(
        "{} cartes, {} nobles",
        catalog.cards.len(),
        catalog.nobles.len()
    );

    for (id, card) in catalog.cards.iter().enumerate() {
        print!(
            "{id:2} | niveau {:?} | bonus {:?} | {} pts | coût ",
            card.level, card.bonus, card.points
        );

        for gem in Gem::ALL {
            let count = card.cost.get(gem);
            if count > 0 {
                print!("{gem:?}:{count} ");
            }
        }
        println!();
    }

    for noble in &catalog.nobles {
        println!("noble | {noble:?}");
    }
}
