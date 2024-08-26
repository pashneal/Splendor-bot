# splendor-tournament
Compete against friends to build the best AI that can beat the board-game Splendor!! Current supports Rust and Python bots!

## MacOS or Linux Installation 

Install python3 and Rust

For installing Rust:
  - visit the official website https://rust-lang.org/tools/install
  - Run the command posted there
  - Restart your terminal

You can check to see if Rust is installed by running `cargo --version` which should have an output that looks like:
```
cargo 1.76.0 (xxxxxx 2024-xxxx)
```


## Windows Installation

### Installing Rust

Navigate to the Rust https://rust-lang.org/tools/install website and download the exe

![image](https://github.com/pashneal/splendor-tournament/assets/36735795/8eebc16a-faa4-4907-b5d4-a9dce6e4b7ab)

When you run it, it may ask you to install Visual Studio Community and Windows SDK. Allow them and click Continue/Install

![image](https://github.com/pashneal/splendor-tournament/assets/36735795/cd0e3d88-1de2-4993-8e30-4a4b9e221f32)

You may get a screen like this, type y and press Enter to continue

![image](https://github.com/pashneal/splendor-tournament/assets/36735795/1b7a726e-1b3d-4be7-bc62-e8ba5f22a100)

After you've installed the SDKs, you'll see a screen like the following, press 1 to install default options and then Enter to continue

![image](https://github.com/pashneal/splendor-tournament/assets/36735795/c33feb93-09da-43a3-b86a-f67a2884d3d2)

If all goes well you should see the following screen 

![image](https://github.com/pashneal/splendor-tournament/assets/36735795/64196d22-f480-42f7-9914-cbe8c6ef3268)

### Python 3 Installation

You may already have Python installed, but the installation scripts require a binary named `python3` to work properly.

To install, you can type `python3` in a new Powershell instance, which will launch Microsoft Store on Windows 11

![image](https://github.com/pashneal/splendor-tournament/assets/42392859/52dd47fd-8c3d-4fbe-8a83-bf52af2cf211)


## Running a Tournament 

To run a tournament is simple, navigate to the `splendor-tournament/scaffolding` directory and execute 
```
python3 run_game.py
```
in your terminal
(Note: the first build can take 3-5 minutes! It should be faster on subsequent builds) 

If all goes well, you should be able to go to http://localhost:3030/splendor/splendor_4pl.html and see the game that the server just played for you.
