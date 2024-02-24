use splendor_tourney::*;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use clap::Parser;

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use splendor_tourney::arena::Arena;

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;


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
    println!("binaries: {:?}", binaries);

    assert!(binaries.len() > 1, "Must have at least two binaries to run a match");
    assert!(binaries.len() < 5, "Cannot have more than 4 binaries to run a match");
    let num_players = binaries.len() as u8;

    Arena::launch(3030, binaries, num_players).await;
}


