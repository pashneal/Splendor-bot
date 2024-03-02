# If you'd like to run a different set of directories, change here!!
# PLAYER_DIRECTORIES = ["python_client/",  "rust_client/", ]
# For example: to run with four players:
PLAYER_DIRECTORIES = ["python_client/",  "rust_client/", "python_client/", "rust_client/"]


# If this is set to True, then report all server errors as well 
# If set to false, only report logs from clients
DEBUG_LOGGING = True

# TODO: Doesn't do anything YET
# Uses Fischer time control conventions, that is, 
# all players start with some INITIAL_TIME given in seconds but before 
# each players next move, INCREMENT seconds is added to that player's total time
INITIAL_TIME = 10
INCREMENT = 5
