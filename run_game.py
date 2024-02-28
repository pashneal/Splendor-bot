# DON'T CHANGE THIS UNLESS YOU REALLY KNOW WHAT YOU'RE DOING!!!
from json import load
import os
import subprocess
from pathlib import Path
import shutil
from config import *

CWD = os.path.dirname(os.path.realpath(__file__)) 
is_windows = os.name.lower() in ["windows", "nt"]

def directory_is_valid(directory):
    if not os.path.exists(directory):
        print(f"COULD NOT RUN: DIRECTORY {directory} was not found")
        return False

    commands_json = os.path.join(directory, "commands.json")
    if not os.path.isfile(commands_json):
        print(f"COULD NOT RUN: {commands_json} was not found")
        return False

    return True


def commands_json_is_valid(filename):

    json = ""

    try:
        with open(filename) as f:
            json = load(f)
    except:
        print(f"COULD NOT RUN: commands.json is not parsable as a json file")
        return False

    if "build" not in json:
        print(f"COULD NOT RUN: 'build' key was not found in commands.json")
        return False
    if type(json["build"]) != list:
        print(f"COULD NOT RUN: 'build' is supposed to be a list of os-agnostic executable commands")
        return False

    for build_command in json["build"]:
        if type(build_command) != str:
            print(f"COULD NOT RUN: all commands in build are supposed to be strings, found {build_command}")
            return False

    if "binary" not in json:
        print(f"COULD NOT RUN: a 'binary' key is missing")
        return False

    if type(json["binary"]) != str:
        print(f"COULD NOT RUN: the 'binary' key should be a single string")
        return False

    return True

def execute_build_commands(json, directory):
    for build_command in json["build"]:
        args = build_command.split(" ")

        os.chdir(directory)
        try:
            subprocess.run(args, check=True)
        except Exception as e:
            os.chdir(CWD)
            print(f"FAILED executing command : {build_command}")
            return False
    os.chdir(CWD)
    return True

def build_arena():
    os.chdir("arena")
    subprocess.run( ("cargo", "build", "--release"), check=True)
    os.chdir(CWD)

def run_game(binaries):
    paths = [str((Path(player_dir)  / Path(binary)).resolve()) for (player_dir, binary) in zip(PLAYER_DIRECTORIES, binaries)]

    build_arena()
    
    if DEBUG_LOGGING:
        os.environ["RUST_LOG"] = "splendor_tourney=trace"
    else:
        os.environ["RUST_LOG"] = "splendor_tourney=info"

    paths = " ".join(paths)
    if is_windows:
        os.system(f".\\arena\\target\\release\\arena -b {paths}")
    else:
        os.system(f"./arena/target/release/arena -b {paths}")

if __name__ == "__main__":
    # Set the current directory to this one to allow for arbitrary execution
    # from any path, but keep relative paths sane
    os.chdir(CWD)

    # TODO: Mark auto loser instead of just crashing
    for player_dir in PLAYER_DIRECTORIES:
        if not directory_is_valid(player_dir):
            print(f"ERROR: {player_dir} directory could not be found. Check your configuration!") 
            exit(1)

    binaries = []
    for player_dir in PLAYER_DIRECTORIES:
        commands = str(Path(player_dir) / Path("commands.json"))
        if not commands_json_is_valid(commands):
            print(f"commands.json {commands} is not valid!!")
            exit(1)

        json = ""
        with open(commands) as f:
            json = load(f)
        if not execute_build_commands(json, player_dir):
            exit(1)

        binaries.append(json["binary"])

    run_game(binaries)
