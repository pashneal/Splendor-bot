import ffi
from ffi import PyCard, PyAction, PyActionType
import random

class Bot:
    def __init__(self, log):
        log.send("Welcome to the explanation of cards")

        # Many types of actions require interaction with some card
        # You can get a list of all the possible cards in the game with:
        cards = PyCard.all_possible_cards()

        # You can initialize a new card with just the ID of a card
        card_61 = PyCard(61)
        card_42 = PyCard(42)

        # actions that require a card can be created with the card_id instead
        if PyAction.purchase(card_id = 61) == PyAction.purchase(card = card_61):
            log.send("[61] They are the same!")

        if PyAction.reserve_face_up(card_id = 42) == PyAction.reserve_face_up(card = card_42):
            log.send("[42] They are the same!")

        # There are also several useful properties of a card to check out
        card = PyCard(23)

        log.send(f"ID {card.id}") # The unique ID of the card with these properties
        log.send(f"TIER {card.tier}") # The tier 0, 1 or 2 that the card is in, 0 is the cheapest
        log.send(f"POINTS {card.points}") # The amount of points the card is worth when played
        log.send(f"GEM_TYPE {card.gem_type}") # The gem associated at the top of the card
        log.send(f"COST {card.cost}") # The cost of the card in total gems


    # This function is called every time the server 
    # needs a move from your bot
    def take_action(self, game_info, log):
        legal_actions = game_info.legal_actions

        # You can grab all the current faceup cards from the game_info
        all_cards = game_info.face_up_cards()

        # Or just the ones in a specific tier
        tier_2_cards = game_info.face_up_cards(tier = 2)
        tier_1_cards = game_info.face_up_cards(tier = 1)
        tier_0_cards = game_info.face_up_cards(tier = 0)

        # You can check out the cards you have reserved
        reserved_cards = game_info.me.reserved_cards
        # Or the gems of the cards you have played
        developments = game_info.me.developments

        total_developments = 0
        # And how many of each gem_type you have played
        total_developments += developments.onyx
        total_developments += developments.sapphire
        total_developments += developments.diamond
        total_developments += developments.emerald
        total_developments += developments.ruby

        log.send(f"Total cards played: {total_developments}")
        log.send(f"Total cards reserved: {len(reserved_cards)}")

        # You can always tell how many cards any single player has reserved
        game_info.players[0].num_reserved_cards
        game_info.players[1].num_reserved_cards

        # Or what types of development cards they have played
        game_info.players[0].developments
        game_info.players[1].developments

        # But be careful! Attempting to peek at reserved_cards from a player
        # that is not you will result in an error which can crash your bot!
        my_index = game_info.me.index
        player = game_info.players[my_index]
        i_can_peek = player.reserved_cards
        log.send(f"Player {my_index} : reserved {i_can_peek}")

        try:
            not_my_index = (my_index + 1) % game_info.num_players
            opponent = game_info.players[not_my_index]
            i_cannot_peek = opponent.reserved_cards
        except:
            log.send("Ooops! You can't look at other players reserved cards!")

        random_action = random.choice(legal_actions)
        return random_action

        
if __name__ == "__main__":
    ffi.run_python_bot(Bot)


