import argparse
import typing
from serde import serde
from serde.json import from_json, to_json
from dataclasses import dataclass
from enum import Enum

from serde import serde, Untagged, AdjacentTagging, InternalTagging


class Color(Enum):
    Onyx = "Onyx"
    Sapphire = "Sapphire"
    Emerald = "Emerald"
    Ruby = "Ruby"
    Diamond = "Diamond"
    Gold = "Gold"

@serde
class Tokens:
    onyx : int
    sapphire : int
    emerald : int
    ruby : int
    diamond : int
    gold : int

@serde 
class Reserve:
    Reserve : int

@serde
class ReserveHidden:
    ReserveHidden : int

@serde
class Purchase:
    Purchase : typing.Tuple[int, Tokens]

@serde
class AttractNoble:
    AttractNoble : int 

@serde
class Discard:
    Discard : typing.List[Color]

@serde
class TakeDistinct:
    TakeDistinct : typing.List[Color]

@serde
class TakeDouble:
    TakeDouble : Color

@serde  
@dataclass
class LogMessage:
    Log : str

@serde(tagging=Untagged)
@dataclass
class ActionMessage:
    Action : typing.Union[
            TakeDistinct, TakeDouble, Purchase, AttractNoble, Discard, Reserve, ReserveHidden]



class Logger: 
    def __init__(self, socket):
        self._socket = socket

    def send(self, message):
        self._socket.send

def get_port():
    parser = argparse.ArgumentParser()
    parser.add_argument('--port', '-p', type=int)
    return parser.parse_args().port

