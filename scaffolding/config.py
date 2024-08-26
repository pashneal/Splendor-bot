# If you'd like to run a different set of directories, change here!!
PLAYER_DIRECTORIES = [
    "../examples/python/actions/",  
    "../examples/python/cards/", 
    "../examples/python/board_and_history", 
    "../examples/python/timeout/",
]

# Rust only examples
# PLAYER_DIRECTORIES = [
    # "../examples/rust/actions/",
    # "../examples/rust/board_and_history/", 
    # "../examples/rust/cards/",
    # "../examples/rust/simple/",
# ]

# For example: to run with four players:
# PLAYER_DIRECTORIES = ["../python_client/",  "../rust_client/", "../python_client/", "../rust_client/"]


# If this is set to True, then report all server errors as well 
# If set to false, only report logs from clients
DEBUG_LOGGING = False

# Uses Fischer time control conventions, that is, 
# all players start with some INITIAL_TIME given in milliseconds but before 
# each players next move, INCREMENT seconds is added to that player's total time
INITIAL_TIME = 1_000
INCREMENT = 1_000
