# splendor-tournament
Compete against friends to build the best AI that can beat the board-game Splendor!! Current supports Rust and Python bots!

## MacOS or Linux Installation 

Install python3 and Rust

For installing Rust:
  - visit the offical website https://rust-lang.org/tools/install
  - Run the command posted there
  - Restart your terminal

You can check to see if Rust is installed by running `cargo --version` which should have an output that looks like:
```
cargo 1.76.0 (xxxxxx 2024-xxxx)
```


## Windows Installation 
TODO

## Running a Tournament 

To run a tournament is simple, navigate to the splendor-tournament directory and execute `python3 run_game.py` in your terminal
(Note: the first build can take 3-5 minutes! It should be faster on subsequent builds) 

If all goes well, you should be able to go to http://localhost:3030/splendor/splendor_4pl.html and see the game that the server just played for you.
