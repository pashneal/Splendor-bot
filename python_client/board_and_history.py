import ffi
from ffi import * 
import random

class Bot:
    def __init__(self, log):
        pass

    def take_action(self, game_info, log):
        # There's lots of useful information in the game_info object!
        num_players = game_info.num_players
        log.send(f"Explanation of a game with {num_players} players")

        # Game information about the current state and the 
        # history of the game until the current point can be accessed
        # like so:

        # You can iterate over the previous states of the game
        history = game_info.history
        all_turns = history.turns
        last_5_turns = all_turns[-5:]

        # You can see what actions were taken during those turns
        for (player_index, actions) in last_5_turns:
            for action in actions:
                if action.action_type == PyActionType.AttractNoble:
                    log.send( f"Careful! Player {player_index} attracted a noble within the last 5 turns!!")
                    log.send( f"They might be close to winning!")

        # You can also see the current state of the game by accessing the board

        board = game_info.board
        # The board has the following attributes:
        board.deck_counts[0] # The number of tier 0 cards face down in the deck
        board.deck_counts[1] # The number of tier 1 cards face down in the deck
        board.deck_counts[2] # The number of tier 2 cards face down in the deck

        board.nobles # The list of nobles that are still available for grabs

        # Or you can check out the cards that are still available
        board.face_up_cards( tier = 0 )
        board.face_up_cards( tier = 1 )
        board.face_up_cards( tier = 2 )

        # Or how many of what type of gem is left in the bank
        board.gems.onyx
        board.gems.sapphire
        board.gems.diamond
        board.gems.emerald
        board.gems.ruby
        board.gems.gold


        if len(board.nobles) > 0:
            log.send(f"There are {len(board.nobles)} nobles left")
            noble = board.nobles[0]

            # You can access the noble's cost
            noble.cost.onyx
            noble.cost.sapphire
            noble.cost.diamond
            noble.cost.emerald
            noble.cost.ruby

            # Or access the noble's id
            noble.id

            # Or create a noble by id
            if PyNoble( noble.id ) == noble:
                log.send("All is right in the noble world...")


        # You can also see the current state of the game by accessing the players
        players = game_info.players
        my_player_index = game_info.me.index
        player = players[my_player_index]

        # You can access the player's various properties
        player.index  # The index of the player (0 up to num_players - 1)

        player.total_points # The number of points the player has scored (including points from nobles)

        player.gems   # The breakdown of gems they have
        player.gems.onyx
        player.gems.sapphire
        player.gems.diamond
        player.gems.emerald
        player.gems.ruby
        player.gems.gold

        player.num_reserved_cards # The number of cards they have reserved 

        player.developments  # The breakdown of the gem type on cards they have played
        player.developments.onyx 
        player.developments.sapphire
        player.developments.diamond
        player.developments.emerald
        player.developments.ruby


        legal_actions = game_info.legal_actions
        action = random.choice(legal_actions)
        return action

if __name__ == "__main__":
    ffi.run_python_bot(Bot)


