use crate::stubs::*;

#[derive(Debug, Default)]
pub struct Bot {
}

impl Runnable<GameInfo, Action> for Bot {
    fn initialize(&mut self, log: &mut Log) {
        log.send("[actions.rs] Welcome to the explanation of actions!");
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
                Action::AttractNoble(noble_id) => {
                    // noble_id is the id of the noble you want to attract
                    // You can see all possible nobles
                    let nobles = Noble::all();
                    // Or figure out which ones you can attract
                    let _attracted_noble = &nobles[*noble_id as usize];
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
