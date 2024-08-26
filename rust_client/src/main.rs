mod examples;
pub use splendor_tourney::stubs::rust as stubs;

use examples::*;
use stubs::run_bot;

fn main() {
    // Check out these bots in the examples folder!
    run_bot::<_, _, simple::Bot>();
    //run_bot::<_, _, cards::Bot>();
    // run_bot::<_, _, actions::Bot>();
    // run_bot::<_, _, board_and_history::Bot>();
}
