import ffi
from ffi import PyAction, PyActionType
import random

class Bot:
    def __init__(self, log):
        log.send("Send instructions at any time")
        log.send(["Hello", "World!"])

        print("But this will not show up, so be careful to use log.send instead")

    # This function is called every time the server 
    # needs a move from your bot
    # You must return a PyAction object
    def take_action(self, game_client_info, log):
        #######################
        # PyAction Examples
        #######################

        # Each of these are PyAction objects
        legal_actions = game_client_info.legal_actions

        # There are usually only four types of legal actions 
        # TakeGems, Reserve, ReserveHidden, Purchase
        # which each have different ways of accessing their data
        for legal_action in legal_actions:

            # You can get the type of the PyAction
            action_type = legal_action.action_type

            # This is when you take gems from the bank
            if action_type == PyActionType.TakeGems:
                # You can access the tokens
                # being taken with this action 
                tokens = legal_action.tokens

                # You can access the number of tokens associated with each with this 
                # specific take gems action
                tokens.onyx         # Black gem
                tokens.sapphire     # Blue gem
                tokens.diamond      # White gem
                tokens.emerald      # Green gem
                tokens.ruby         # Red gem
                tokens.gold         # Yellow gem

                # You can also create your own TakeGems actions
                action = PyAction.take_gems( onyx = 1, ruby = 1, sapphire = 1 )
                action = PyAction.take_gems( onyx = 2 )
                
                # You can check if the action you created is equal to another
                PyAction.take_gems( onyx = 2 ) == PyAction.take_gems( onyx = 2 , ruby = 0 )
                # or in a list
                if PyAction.take_gems( onyx = 2 ) in legal_actions:
                    pass

                return legal_action

            # This is when you take a card from the faceup set of cards
            if action_type == PyActionType.ReserveFaceUp:
                # You can access the card_id of the card being reserved
                card_id = legal_action.card_id

                # You can also create your own Reserve actions
                action = PyAction.reserve_face_up( card_id = 0 )

                return legal_action

            # This is when you take a card from the top of the facedown
            # deck of cards
            if action_type == PyActionType.ReserveFaceDown:
                # You can access the tier being reserved
                # 0 is the first tier
                # 1 is the second tier
                # 2 is the third tier
                tier = legal_action.tier

                # You can also create your own ReserveFaceDown actions
                action = PyAction.reserve_face_down( tier = 2 )

                return legal_action


            # This is when you purchase a card from your hand (or the board)
            if action_type == PyActionType.Purchase:
                # You can access the card_id of the card being purchased
                card_id = legal_action.card_id
                # And which specific tokens you'll be using to pay for it
                tokens = legal_action.tokens

                # You can also create your own Purchase actions
                action = PyAction.purchase( card_id = 43, 
                                   onyx = 1, 
                                   ruby = 1, 
                                   sapphire = 1, 
                                   diamond = 1, 
                                   emerald = 1,
                                   gold = 3)

                return legal_action

        # There are also two special actions:
        #
        # Discard - when you've taken too many tokens and must discard back down to 10
        #
        # AttractNoble - when you have enough cards to attract multiple nobles
        #                and must choose which one to attract. (note
        #                that if you only attract one noble, it will be done automatically
        #                for you)
        for legal_action in legal_actions:
            # This is when you discard excess gems from your hand
            # back to the bank
            if action_type == PyActionType.Discard:
                # You can access the tokens being discarded
                tokens = legal_action.tokens

                # You can also create your own Discard actions
                action = PyAction.discard( onyx = 1, ruby = 1, sapphire = 1 )

                return legal_action

            # This is when you attract a noble to your hand
            if action_type == PyActionType.AttractNoble:
                # You can access the noble_id of the noble being attracted
                noble_id = legal_action.noble_id

                # You can also create your own AttractNoble actions
                action = PyAction.attract_noble( noble_id = 0 )

                return legal_action
         

        # You aren't allowed to pass a turn, 
        # if you only have one legal action, the server will automatically
        # play it for you
        some_random_action = random.choice(legal_actions)
        log.send("I'm taking a random action: " + str(some_random_action))
        return some_random_action


if __name__ == "__main__":
    ffi.run_python_bot(Bot)

 
