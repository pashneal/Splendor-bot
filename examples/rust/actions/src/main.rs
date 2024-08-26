pub use splendor_tourney::stubs::rust as stubs;

use stubs::*;

#[derive(Debug, Default)]
pub struct Bot {}

impl Runnable<GameInfo, Action> for Bot {
    fn initialize(&mut self, log: &mut Log) {
        log.send("[actions] Welcome to the explanation of actions!");
    }

    fn take_action(&mut self, info: GameInfo, _log: &mut Log) -> Action {
        let legal_actions = &info.legal_actions;

        for action in legal_actions {
            match action {
                Action::Purchase(card_id, gems) => {
                    // card_id is the id of the card you want to purchase
                    // gems the way you're going to pay for it

                    // You can purchase any faceup card or any reserved card
                    // from your hand that you can afford
                    let _reserved_cards = &info.me().reserved_cards.clone().unwrap();

                    // You can see the breakdown
                    gems.onyx;
                    gems.sapphire;
                    gems.diamond;
                    gems.emerald;
                    gems.ruby;
                    gems.gold;

                    // Or create your own
                    let my_gems = Gems {
                        onyx: 1,
                        sapphire: 1,
                        emerald: 1,
                        ruby: 1,
                        diamond: 1,
                        gold: 0,
                    };
                    Action::Purchase(*card_id, my_gems);
                }
                Action::ReserveFaceUp(_card_id) => {
                    // card_id is the id of the card you want to reserve
                }
                Action::ReserveFaceDown(_tier) => {
                    // tier is the tier of the card you want to reserve
                    // 0, 1, or 2
                    // 0 is the cheapest, 2 is the most expensive tier
                }
                Action::TakeGems(_gems) => {
                    // gems is the way you want to take gems,
                    // you can take 3 different gems or 2 of the same
                    // (with some exceptions)
                }
                Action::DiscardGems(_gems) => {
                    // gems is the way you want to discard gems
                    // This is only possible if you have more than 10 gems
                    // you must discard down to 10
                }
                Action::AttractNoble(noble) => {
                    // noble has a few interesting fields that you can use
                    noble.id;
                    noble.points; // Always 3 btw
                    let gems = &noble.requirements();

                    noble.is_attracted_to(&info.me().developments);

                    gems.onyx;
                    gems.sapphire;
                    gems.emerald;
                    gems.ruby;
                    gems.diamond;

                    let _all_nobles = Noble::all();
                    let _board_nobles = &info.board.nobles;
                }
                Action::Pass => {
                    // Here for completeness, you cannot choose to pass
                    // the server decides for you
                }
                Action::Continue => {
                    // Here for completeness, you cannot choose to continue
                    // the server decides for you
                }
            }
        }

        legal_actions[0].clone()
    }
}

fn main() {
    // Check out these bots in the examples folder!
    run_bot::<_, _, Bot>();
}
