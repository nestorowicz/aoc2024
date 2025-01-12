import os

def debug(arg):
    if "DEBUG" in os.environ:
        print(arg)
