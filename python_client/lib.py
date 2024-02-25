import argparse
from typing import Union, Tuple, List, NewType
from plum import dispatch
from serde import serde, field
from serde.json import from_json, to_json
from dataclasses import dataclass
from enum import Enum


from serde import serde, Untagged, InternalTagging



class Color(Enum):
    Onyx = "Onyx"
    Sapphire = "Sapphire"
    Emerald = "Emerald"
    Ruby = "Ruby"
    Diamond = "Diamond"
    Gold = "Gold"


@serde
class Tokens:
    onyx: int
    sapphire: int
    emerald: int
    ruby: int
    diamond: int
    gold: int

@serde
class Cost:
    onyx: int
    sapphire: int
    emerald: int
    ruby: int
    diamond: int

@serde
class Reserve:
    Reserve: int


@serde
class ReserveHidden:
    ReserveHidden: int


@serde
class Purchase:
    Purchase: Tuple[int, Tokens]


@serde
class AttractNoble:
    AttractNoble: int


@serde
class Discard:
    Discard: List[Color]


@serde
class TakeDistinct:
    TakeDistinct: List[Color]


@serde
class TakeDouble:
    TakeDouble: Color

@serde
class Pass:
    pass

@serde
@dataclass
class LogMessage:
    Log: str

Actions = Union[
        TakeDistinct,
        TakeDouble,
        Purchase,
        AttractNoble,
        Discard,
        Reserve,
        ReserveHidden,
        Pass
    ]


def action_serialize(value: Actions):
    json = to_json(value)
    if json == "{}":
        return value.__class__.__name__
    return json


@serde(tagging=Untagged)
class ActionType:
    Action: Actions = field(serializer = action_serialize)

@serde(tagging=Untagged)
@dataclass
class ActionMessage:
    Action: ActionType


@serde
class Board:
    deck_counts : Tuple[int, int, int]
    available_cards : List[List[int]]
    nobles : List[int]
    tokens : Tokens

@serde
class GameHistory:
    history : List[Tuple[int, ActionType]]

@serde
class PlayerPublicInfo:
    points: int
    num_reserved : int
    developments : Cost
    gems : Tokens

@serde
class Player:
    points: int
    reserved : List[int]
    gems: Tokens
    developments : Tokens
    blind_reserved : List[int]
    
@serde
class ClientInfo:
    board: Board
    history : GameHistory
    players : List[PlayerPublicInfo]
    current_player : Player
    current_player_num: int
    legal_actions: List[ActionMessage]


class Logger:
    def __init__(self, socket):
        self._socket = socket

    def send(self, message):
        self._socket.send


def get_port():
    parser = argparse.ArgumentParser()
    parser.add_argument("--port", "-p", type=int)
    return parser.parse_args().port

if __name__ == "__main__":
    test_info = ClientInfo(
        board=Board(
            deck_counts=(1, 2, 3),
            available_cards=[[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            nobles=[1, 2, 3],
            tokens=Tokens(1, 2, 3, 4, 5, 6),
        ),
        history=GameHistory(history=[]),
        players=[
            PlayerPublicInfo(
                points=1,
                num_reserved=2,
                developments=Cost(1, 2, 3, 4, 5),
                gems=Tokens(1, 2, 3, 4, 5, 6),
            )
        ],
        current_player=Player(
            points=1,
            reserved=[1, 2, 3],
            gems=Tokens(1, 2, 3, 4, 5, 6),
            developments=Tokens(1, 2, 3, 4, 5, 6),
            blind_reserved=[1, 2, 3],
        ),
        current_player_num=1,
        legal_actions=[ActionMessage(TakeDistinct([Color.Onyx, Color.Sapphire, Color.Emerald]))],
    )

    print(to_json(test_info))

    print(to_json(ActionType(TakeDistinct([Color.Onyx, Color.Sapphire, Color.Emerald]))))
    # json = """{"board":{"deck_counts":[36,26,15],"available_cards":[[36,19,35,14],[48,60,43,41],[83,89,88,82]],"nobles":[3,7,0],"tokens":{"onyx":4,"sapphire":3,"emerald":4,"ruby":3,"diamond":3,"gold":4}},"history":{"history":[[0,{"TakeDistinct":["Sapphire","Ruby","Diamond"]}],[0,"Pass"],[0,"Continue"],[1,{"Reserve":76}],[1,"Pass"],[1,"Continue"]]},"players":[{"points":0,"num_reserved":0,"developments":{"onyx":0,"sapphire":0,"emerald":0,"ruby":0,"diamond":0},"gems":{"onyx":0,"sapphire":1,"emerald":0,"ruby":1,"diamond":1,"gold":0}},{"points":0,"num_reserved":1,"developments":{"onyx":0,"sapphire":0,"emerald":0,"ruby":0,"diamond":0},"gems":{"onyx":0,"sapphire":0,"emerald":0,"ruby":0,"diamond":0,"gold":1}}],"current_player":{"points":0,"reserved":[],"gems":{"onyx":0,"sapphire":1,"emerald":0,"ruby":1,"diamond":1,"gold":0},"developments":{"onyx":0,"sapphire":0,"emerald":0,"ruby":0,"diamond":0,"gold":0},"blind_reserved":[]},"current_player_num":0,"legal_actions":[{"ReserveHidden":0},{"Reserve":36},{"Reserve":19},{"Reserve":35},{"Reserve":14},{"ReserveHidden":1},{"Reserve":48},{"Reserve":60},{"Reserve":43},{"Reserve":41},{"ReserveHidden":2},{"Reserve":83},{"Reserve":89},{"Reserve":88},{"Reserve":82},{"TakeDistinct":["Sapphire","Onyx","Diamond"]},{"TakeDistinct":["Sapphire","Emerald","Onyx"]},{"TakeDistinct":["Diamond","Emerald","Sapphire"]},{"TakeDistinct":["Ruby","Diamond","Emerald"]},{"TakeDistinct":["Ruby","Diamond","Onyx"]},{"TakeDistinct":["Ruby","Onyx","Emerald"]},{"TakeDistinct":["Sapphire","Onyx","Ruby"]},{"TakeDistinct":["Diamond","Sapphire","Ruby"]},{"TakeDistinct":["Onyx","Diamond","Emerald"]},{"TakeDistinct":["Sapphire","Emerald","Ruby"]},{"TakeDouble":"Onyx"},{"TakeDouble":"Emerald"}]}
# """
    # print(from_json(ClientInfo, json))





