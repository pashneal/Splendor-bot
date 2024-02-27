import ffi
print(ffi.multiply(3,4))
print(ffi.test_from_json("""{"board":{"deck_counts":[35,26,15],"available_cards":[[13,6,26,32],[62,51,57,45],[80,71,85,78]],"nobles":[1,3,6],"tokens":{"onyx":4,"sapphire":4,"emerald":4,"ruby":4,"diamond":4,"gold":3}},"history":{"history":[[0,{"Reserve":5}],[0,"Pass"],[0,"Continue"],[1,{"Reserve":84}],[1,"Pass"],[1,"Continue"]]},"players":[{"points":0,"num_reserved":1,"developments":{"onyx":0,"sapphire":0,"emerald":0,"ruby":0,"diamond":0},"gems":{"onyx":0,"sapphire":0,"emerald":0,"ruby":0,"diamond":0,"gold":1}},{"points":0,"num_reserved":1,"developments":{"onyx":0,"sapphire":0,"emerald":0,"ruby":0,"diamond":0},"gems":{"onyx":0,"sapphire":0,"emerald":0,"ruby":0,"diamond":0,"gold":1}}],"current_player":{"points":0,"reserved":[5],"gems":{"onyx":0,"sapphire":0,"emerald":0,"ruby":0,"diamond":0,"gold":1},"developments":{"onyx":0,"sapphire":0,"emerald":0,"ruby":0,"diamond":0,"gold":0},"blind_reserved":[]},"current_player_num":0,"legal_actions":[{"ReserveHidden":0},{"Reserve":13},{"Reserve":6},{"Reserve":26},{"Reserve":32},{"ReserveHidden":1},{"Reserve":62},{"Reserve":51},{"Reserve":57},{"Reserve":45},{"ReserveHidden":2},{"Reserve":80},{"Reserve":71},{"Reserve":85},{"Reserve":78},{"TakeDistinct":["Onyx","Sapphire","Diamond"]},{"TakeDistinct":["Onyx","Ruby","Emerald"]},{"TakeDistinct":["Onyx","Ruby","Diamond"]},{"TakeDistinct":["Sapphire","Emerald","Ruby"]},{"TakeDistinct":["Ruby","Onyx","Sapphire"]},{"TakeDistinct":["Sapphire","Emerald","Onyx"]},{"TakeDistinct":["Ruby","Diamond","Sapphire"]},{"TakeDistinct":["Sapphire","Emerald","Diamond"]},{"TakeDistinct":["Diamond","Emerald","Ruby"]},{"TakeDistinct":["Diamond","Onyx","Emerald"]},{"TakeDouble":"Onyx"},{"TakeDouble":"Sapphire"},{"TakeDouble":"Emerald"},{"TakeDouble":"Ruby"},{"TakeDouble":"Diamond"}]}
""").legal_actions[0])

class A:
    def __init__(self):
        self.a = 4
    def test(self):
        return self.a
print(ffi.test_init_call(A))


class Bot:
    def __init__(self):
        pass

    def initialize(self, log):
        log.send("Hi")

    def take_action(self, clientinfo, log):
        log.send("Hi2")
        return clientinfo.legal_actions[0]

print(ffi.run_python_bot(Bot))


