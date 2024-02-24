use splendor_tourney::*;
use std::sync::Arc;
use tungstenite::{connect, Message, stream::MaybeTlsStream};
use url::Url;
use rand::{thread_rng, seq::SliceRandom};
use clap::Parser;

type WebSocket = tungstenite::WebSocket<MaybeTlsStream<std::net::TcpStream>>;

/// Your bot struct, which will live for the duration of the game
/// Feel free to add any fields you need, but they must each implement Default!
#[derive(Debug, Default)]
pub struct Bot {
    pub name : String,
    pub turns : usize,
}

/// Initialize your bot here!
/// feel free to change around the items in your Bot struct
/// This is called *once* at the start of a new game
pub fn initialize(bot: &mut Bot, log: &mut Log) {
    // The stuff in bot carries over from turn to turn!
    bot.name = "Cool bot name".to_string();
    bot.turns = 0;

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
pub fn take_action(bot: &mut Bot, info: ClientInfo, log : &mut Log) -> Action {
    let legal_actions = info.legal_actions;

    // Just choose a random action (this bot is not very smart)
    let mut rng = thread_rng();
    let action = legal_actions.choose(&mut rng).unwrap();

    bot.turns += 1;
    let message = format!("I chose to {:?} and I have seen {} turns! Take that!", action, bot.turns);

    // Note: nothing will print to the console, 
    println!("This does not print out!!");
    // use the log instead so that the server prints it for you
    log.send(&message);

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

pub struct Log {
    socket : WebSocket,
}

impl Log {
    pub fn new(port: u16) -> Self {
        let url = format!("ws://localhost:{}/log", port);
        let url = Url::parse(&url).unwrap();
        let (socket, _) = connect(url).expect("Can't connect to the game server");
        Self {
            socket,
        }
    }

    pub fn send(&mut self, message: &str) {
        let message = ClientMessage::Log(message.to_string());
        let message = serde_json::to_string(&message).expect("Error converting message to string");
        self.socket.send(Message::Text(message)).expect("Error writing message");
    }
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let port = args.port;

    let url = format!("ws://localhost:{}/game", port);
    let url = Url::parse(&url).unwrap();
    let (mut game_socket, _) = connect(url).expect("Can't connect to the game server");

    // Give the server a chance to start up
    std::thread::sleep(std::time::Duration::from_millis(100));

    let mut log = Log::new(port);

    let mut bot = Bot::default();
    initialize(&mut bot, &mut log);
    println!("Connected to the game server...");

    loop {
        let msg = game_socket.read().expect("Error reading message");
        let msg = msg.to_text().expect("Error converting message to text");
        let info: ClientInfo = serde_json::from_str(msg).expect("Error parsing message");
        let action = take_action(&mut bot, info, &mut log);
        let msg = ClientMessage::Action(action);

        let msg_str = serde_json::to_string(&msg).expect("Error converting action to string");
        game_socket.send(Message::Text(msg_str)).expect("Error sending message");

    }
}
