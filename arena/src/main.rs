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
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let binaries = args.binaries;
    let port = args.port;
    println!("binaries: {:?}", binaries);

    assert!(binaries.len() > 1, "Must have at least two binaries to run a match");
    assert!(binaries.len() < 5, "Cannot have more than 4 binaries to run a match");
    let num_players = binaries.len() as u8;

    Arena::launch(port, binaries, num_players).await;
}


