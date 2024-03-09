use rand::{seq::SliceRandom, thread_rng};
use crate::stubs::*;

/// Your bot struct, which will live for the duration of the game
/// Feel free to add any fields you need, but they must implement Default!
#[derive(Debug, Default)]
pub struct Bot {
    pub name: String,
    pub turn_counter: usize,
}

impl Runnable<GameInfo, Action> for Bot {
    /// Initialize your bot here!
    /// feel free to change around the items in your Bot struct
    /// This is called *once* at the start of a new game
    fn initialize(&mut self, log: &mut Log) {
        // The stuff in bot carries over from turn to turn!
        self.name = "Cool bot name".to_string();
        self.turn_counter = 0;

        // Send a message from this bot to the server for debugging!
        log.send("Hello from a new bot!");
    }

    /// The main bread and butter of your bot!
    ///
    /// This is called whenever the game needs to take an action from your bot,
    /// the actions that you can take are given in the GameInfo.legal_actions field
    ///
    /// If you have <= 1 legal action, the server will decide for you
    /// and skip this function. This includes attracting a single noble.
    fn take_action(&mut self, info: GameInfo, log: &mut Log) -> Action {
        let legal_actions = info.legal_actions;

        // Just choose a random action (this bot is not very smart)
        let mut rng = thread_rng();
        let action = legal_actions.choose(&mut rng).unwrap();
        self.turn_counter += 1;

        action.clone()
    }
}
