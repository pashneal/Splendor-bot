import ffi
import random

class Bot:
    def __init__(self, log):
        log.send("Hello, Python bot here and online!!")

    def take_action(self, game_info, log):
        action = random.choice(game_info.legal_actions)
        log.send("Time Remaining: " + str(game_info.time_remaining()) +  " milliseconds")
        log.send("Hmmmm... I'm taking this action: " + str(action))
        return action

if __name__ == "__main__":
    ffi.run_python_bot(Bot)


