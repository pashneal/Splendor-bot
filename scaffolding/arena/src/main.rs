use clap::Parser;
use splendor_tourney::Arena;


#[derive(Parser, Debug)]
pub struct Args{
    /// The port to run the server on
    #[arg(short, long, default_value = "3030")]
    port: u16,
    /// The binaries to run in the tournament
    #[arg(short, long, num_args(0..))]
    binaries: Vec<String>,
    /// The initial time in milliseconds (according to fischer time control) 
    /// default is 10_000ms
    #[arg(short, long, default_value="10_000")]
    initial_time: u64,
    /// The increment in milliseconds (according to fischer time control)
    /// default is 1_000ms
    #[arg(short, long, default_value="1_000")]
    increment: u64,

}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let binaries = args.binaries;
    let port = args.port;
    let initial_time = std::time::Duration::from_millis(args.initial_time);
    let increment = std::time::Duration::from_millis(args.increment);

    println!("binaries: {:?}", binaries);

    assert!(binaries.len() > 1, "Must have at least two binaries to run a match");
    assert!(binaries.len() < 5, "Cannot have more than 4 binaries to run a match");
    let num_players = binaries.len() as u8;

    Arena::launch(port, binaries, num_players, initial_time, increment).await;
}


