import ffi
import random

class Bot:
    def __init__(self, log):
        log.send("Hello, Python bot here and online!!")

    def take_action(self, game_client_info, log):
        action = random.choice(game_client_info.legal_actions)
        log.send("Hmmmm... I'm taking this action: " + str(action))
        return action

ffi.run_python_bot(Bot)


