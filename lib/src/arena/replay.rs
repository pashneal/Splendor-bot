use super::*;
use warp::{Reply, Rejection, Filter};
use std::marker::PhantomData;
use crate::nobles::Noble;
use std::collections::HashMap;
use crate::gem_type::GemType;
use log::trace;
use crate::card::CardId;

// Note: the following code results from me playing around with 
//
// 1) a type stating system to have illegal state be unrepresentable
// 2) an sum type to represent either of two state in the same field
//
// It is not exactly the cleanest code, but I wanted
// to play around with these ergonomics

#[derive(Debug, Clone)]
pub enum Either<A, B> {
    Initialized(A),
    Finalized(B),
}

pub trait ReplayState {}
impl ReplayState for Initialized {}
impl ReplayState for Finalized {}

#[derive(Debug, Clone)]
pub struct Initialized {
    initial_game : Game,
}
#[derive(Debug, Clone)]
pub struct Finalized {
    initial_game : Game,
    viewable_game : Game,
    history : GameHistory,
    move_index : usize,
}

#[derive(Debug, Clone)]
pub struct Replay<T : ReplayState> {
    inner : T
}

impl Replay<Initialized> {
    pub fn new(game : Game) -> Replay<Initialized> {
        Replay {
            inner : Initialized {
                initial_game : game,
            }
        }
    }

    pub fn finalize_with(self, history : GameHistory) -> Replay<Finalized> {
        Replay {
            inner : Finalized {
                initial_game : self.inner.initial_game.clone(),
                viewable_game : self.inner.initial_game.clone(),
                history, 
                move_index : 0,
            }
        }
    } 
}

impl Replay<Finalized> {
    pub fn next_move(&mut self) {
        self.go_to_move(self.inner.move_index as i32 + 1)
    }

    pub fn previous_move(&mut self) {
        self.go_to_move(self.inner.move_index as i32 - 1)
    }

    pub fn go_to_move(&mut self, new_move_index : i32) {
        // Bound between 0 and the number of moves not matter the input
        let new_move_index = new_move_index.max(0);
        let new_move_index = new_move_index.min(self.inner.history.num_moves());

        self.inner.move_index = new_move_index as usize;

        // Replay the game up to the given number
        let history = self.inner.history.take_until_move(new_move_index);
        trace!("Replaying history : {:?}", history);
        let mut init_game = self.inner.initial_game.clone();
        init_game.advance_history_with(history);



        self.inner.viewable_game = init_game;
    }

    pub fn current_game(&self) -> &Game {
        &self.inner.viewable_game
    }
}

pub type FinalizedReplay = Arc<RwLock<Replay<Finalized>>>;


// (color/gem, amount)
type JSTokens = Vec<(usize, i8)>;

#[derive(Debug, Clone, Serialize)]
pub struct JSCard {
    tier : usize,
    points : usize,
    #[serde(rename = "colorIndex")]
    color_index : usize,
    tokens : JSTokens,
}


#[derive(Debug, Serialize)]
enum Success {
    #[serde(rename = "move_index")]
    Move(usize),
    #[serde(rename = "nobles")]
    Nobles(Vec<JSTokens>),
    #[serde(rename = "cards")]
    Cards(Vec<Vec<JSCard>>),
}

#[derive(Debug, Serialize)]
enum EndpointReply{
    #[serde(rename = "success")]
    Success(Success),
    #[serde(rename = "error")]
    Error(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Move {
    pub move_index : i32,
}

pub fn json_body() -> impl Filter<Extract = (Move,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn next_move( arena : GlobalArena) -> Result<impl Reply, Rejection> {
    let replay = arena.write().await.get_replay();
    match replay {
        None => Ok(warp::reply::json(&EndpointReply::Error("No replay available".to_string()))),
        Some(replay) => {
            replay.write().await.next_move();
            let move_index = replay.read().await.inner.move_index;
            Ok(warp::reply::json(&EndpointReply::Success(Success::Move(move_index))))
        }
    }
}

pub async fn previous_move( arena : GlobalArena) -> Result<impl Reply, Rejection> {
    let replay = arena.write().await.get_replay();
    match replay {
        None => Ok(warp::reply::json(&EndpointReply::Error("No replay available".to_string()))),
        Some(replay) => {
            replay.write().await.previous_move();
            let move_index = replay.read().await.inner.move_index;
            Ok(warp::reply::json(&EndpointReply::Success(Success::Move(move_index))))
        }
    }
}

pub async fn go_to_move(move_number : Move, arena : GlobalArena ) -> Result<impl Reply, Rejection> {
    let replay = arena.write().await.get_replay();
    match replay {
        None => Ok(warp::reply::json(&EndpointReply::Error("No replay available".to_string()))),
        Some(replay) => {
            let move_number = move_number.move_index;
            replay.write().await.go_to_move(move_number);
            let move_index = replay.read().await.inner.move_index;
            Ok(warp::reply::json(&EndpointReply::Success(Success::Move(move_index))))
        }
    }
}

// Match the conventions of the frontend gems
//
//          color    : index 
//	 white (diamond) : 0
//	 blue (sapphire) : 1
//	 green (emerald) : 2 
//	 red (ruby)      : 3
//	 black (onyx)    : 4
//	 yellow (gold)   : 5
fn js_gems_map() -> HashMap<GemType, usize> {
    let mut map = HashMap::new();
    map.insert(GemType::Diamond, 0);
    map.insert(GemType::Sapphire, 1);
    map.insert(GemType::Emerald, 2);
    map.insert(GemType::Ruby, 3);
    map.insert(GemType::Onyx, 4);
    map.insert(GemType::Gold, 5);
    map
}

// Converts a noble to a vector representing the color distribution
// of the cost of the noble as a list of (color_index, number_needed)
fn to_js_noble(noble : &Noble) -> JSTokens {

    let mut map = js_gems_map(); 
    let mut js_noble = Vec::new();

    let tokens = noble.requirements();

    for gem in GemType::all_expect_gold() {
        let index = map.get(&gem).unwrap();
        let count = tokens[gem];
        if count > 0 {
            js_noble.push((*index, count));
        }
    }

    js_noble
}

pub async fn board_nobles( arena : GlobalArena) -> Result<impl Reply, Rejection> {
    let replay = arena.write().await.get_replay();
    match replay {
        None => Ok(warp::reply::json(&EndpointReply::Error("No replay available".to_string()))),
        Some(replay) => {
            trace!("Getting nobles");
            let game = &replay.read().await.inner.viewable_game;
            let nobles = game.nobles();
            trace!("Got nobles : {:#?}", nobles);
            let js_nobles = nobles.iter().map(|n| to_js_noble(&n)).collect();
            Ok(warp::reply::json(&EndpointReply::Success(Success::Nobles(js_nobles))))
        }
    }
}


fn to_js_cards(card_ids : Vec<Vec<CardId>>, card_lookup : Arc<Vec<Card>>)  -> Vec<Vec<JSCard>> {
    let cards = card_ids.iter().flatten().map(|&c| card_lookup[c as usize].clone()).collect::<Vec<Card>>();
    let map = js_gems_map();
    let js_cards : Vec<JSCard> = cards.iter().map(|c| {
        let tier = (c.tier() - 1) as usize;
        let points = c.points() as usize;
        let cost = c.cost();
        let mut js_cost = Vec::new();

        for gem in GemType::all_expect_gold() {
            let index = map.get(&gem).unwrap();
            let count = cost[gem];
            if count > 0 {
                js_cost.push((*index, count));
            }
        }

        let color_index = map.get(&c.gem()).unwrap();

        JSCard {
            tier,
            points,
            color_index : *color_index,
            tokens : js_cost,
        }
    }).collect();

    // Group by tier
    let mut grouped = vec![Vec::new(); 3];
    for card in js_cards {
        grouped[card.tier].push(card);
    }

    grouped

}

pub async fn board_cards( arena : GlobalArena) -> Result<impl Reply, Rejection> {
    let replay = arena.write().await.get_replay();
    match replay {
        None => Ok(warp::reply::json(&EndpointReply::Error("No replay available".to_string()))),
        Some(replay) => {
            let card_lookup = replay.read().await.inner.viewable_game.card_lookup();
            let cards = replay.read().await.inner.viewable_game.cards();
            let js_cards = to_js_cards(cards, card_lookup);
            Ok(warp::reply::json(&EndpointReply::Success(Success::Cards(js_cards))))
        }
    }
}
