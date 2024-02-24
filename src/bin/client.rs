use splendor_tourney::*;
use std::sync::Arc;
use tungstenite::{connect, Message};
use url::Url;
use rand::{thread_rng, seq::SliceRandom};
use clap::Parser;


/// Your bot struct, which will live for the duration of the game
/// Feel free to add any fields you need, but they must each implement Default!
#[derive(Debug, Default)]
pub struct Bot {
    pub message : String,
}

/// Initialize your bot here!
/// feel free to change around the items in your Bot struct
/// This is called *once* at the start of a new game
pub fn initialize(bot: &mut Bot) {
    bot.message = "Hello world".to_string();
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
pub fn take_action(bot: &mut Bot, info: ClientInfo) -> Action {
    let legal_actions = info.legal_actions;

    // Just choose a random action (this bot is not very smart)
    let mut rng = thread_rng();
    let action = legal_actions.choose(&mut rng).unwrap();

    let message = format!("I chose to {:?}! Take that!", action);
    println!("{}", message);

    action.clone()
}


/// This is called at the end of the game, and you can use it to clean up
/// TODO: Actually call this from the server when the game is over
pub fn game_over(bot: &mut Bot, info: ClientInfo, results: GameResults) {
    todo!()
}


#[derive(Parser, Debug)]
pub struct Args {
    /// The port to connect to
    #[arg(short, long)]
    port: u16,

}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let port = args.port;

    let url = format!("ws://localhost:{}/game", port);
    let url = Url::parse(&url).unwrap();
    let (mut socket, _) = connect(url).expect("Can't connect to the game server");

    let mut bot = Bot::default();
    initialize(&mut bot);
    println!("Connected to the game server...");

    loop {
        let msg = socket.read().expect("Error reading message");
        let msg = msg.to_text().expect("Error converting message to text");
        let info: ClientInfo = serde_json::from_str(msg).expect("Error parsing message");
        let action = take_action(&mut bot, info);

        let action_str = serde_json::to_string(&action).expect("Error converting action to string");
        socket.send(Message::Text(action_str)).expect("Error sending message");

    }
}
