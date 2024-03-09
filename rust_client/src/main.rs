mod stubs;

use stubs::*;
use rand::{thread_rng, seq::SliceRandom};

/// Your bot struct, which will live for the duration of the game
/// Feel free to add any fields you need, but they must implement Default!
#[derive(Debug, Default)]
pub struct Bot {
    pub name : String,
    pub turn_counter : usize,
}

impl Runnable<ClientInfo, Action> for Bot {

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
    /// the actions that you can take are given in the ClientInfo.legal_actions field
    ///
    /// Generally, you'll have different types of actions you can take
    ///     -> Reserve or Purchase cards or Take gems
    ///     -> Discard gems (if you have too many before the end of your turn)
    ///     -> Obtain nobles  (if you have a choice between several nobles)
    ///
    /// If you have <= 1 legal action, the server will decide for you
    /// and skip this function. This includes attracting a single noble.
    fn take_action(&mut self, info: ClientInfo, log : &mut Log) -> Action {
        let legal_actions = info.legal_actions;

        // Just choose a random action (this bot is not very smart)
        let mut rng = thread_rng();
        let action = legal_actions.choose(&mut rng).unwrap();

        self.turn_counter += 1;
        let message = format!("I chose to {:?} and I have seen {} turns! Take that!", action, self.turn_counter);

        // Note: nothing will print to the console, 
        println!("This does not print out!!");
        // use the log instead so that the server prints it for you
        log.send(&message);

        action.clone()
    }

    /// This is called at the end of the game, and you can use it to clean up
    /// TODO: Actually call this from the server when the game is over
    fn game_over(&self, _info: ClientInfo, _results: GameResults) {
        todo!()
    }
}

fn main() {
    run_bot::<_,_,Bot>()
}
