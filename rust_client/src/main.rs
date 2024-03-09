mod examples;
mod stubs;

use examples::*;
use stubs::run_bot;


fn main() {
    // Check out these bots in the examples folder!
    run_bot::<_, _, simple::Bot>();
    //run_bot::<_, _, cards::Bot>();
    //run_bot::<_, _, actions::Bot>();
}
