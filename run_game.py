# DON'T CHANGE THIS UNLESS YOU REALLY KNOW WHAT YOU'RE DOING!!!
from json import load
import os
import subprocess
from pathlib import Path
import shutil
from config import *

CWD = os.path.dirname(os.path.realpath(__file__)) 

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

def run_game(binary0, binary1):
    path0 = str((Path(PLAYER_0_DIRECTORY)  / Path(binary0)).resolve())
    path1 = str((Path(PLAYER_1_DIRECTORY)  / Path(binary1)).resolve())

    build_arena()
    
    if DEBUG_LOGGING:
        os.environ["RUST_LOG"] = "splendor_tourney=trace"
    else:
        os.environ["RUST_LOG"] = "splendor_tourney=info"

    os.system(f"./arena/target/release/arena -b {path0} {path1}")

if __name__ == "__main__":
    # TODO: Mark auto loser instead of just crashing
    if not directory_is_valid(PLAYER_0_DIRECTORY):
        print("PLAYER_0_DIRECTORY variable is not set to a valid directory")
        exit(1)
    if not directory_is_valid(PLAYER_1_DIRECTORY):
        print("PLAYER_1_DIRECTORY variable is not set to a valid directory")
        exit(1)

    commands0 = str((Path(PLAYER_0_DIRECTORY) / Path("./commands.json")).resolve())
    commands1 = str((Path(PLAYER_1_DIRECTORY) / Path("./commands.json")).resolve())
    print(commands1)

    if not commands_json_is_valid(commands0):
        print(f"commands.json {commands0} is not valid!!")
        exit(1)
    if not commands_json_is_valid(commands1):
        print(f"commands.json {commands1} is not valid!!")
        exit(1)

    json0 = ""
    json1 = ""
    with open(commands0) as f:
        json0 = load(f)
    with open(commands1) as f:
        json1 = load(f)


    if not execute_build_commands(json0, PLAYER_0_DIRECTORY):
        exit(1)
    if not execute_build_commands(json1, PLAYER_1_DIRECTORY):
        exit(1)

    run_game(json0["binary"], json1["binary"])
