import ffi
from ffi import PyCard, PyAction, PyActionType
import random
import time

class Bot:
    def __init__(self, log):
        log.send("See something")

    # This function is called every time the server 
    # needs a move from your bot
    def take_action(self, game_info, log):
        time.sleep(1.2)
        return game_info.legal_actions[0]
        
if __name__ == "__main__":
    ffi.run_python_bot(Bot)


