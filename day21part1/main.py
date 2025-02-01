#!/usr/bin/env python3
from collections import namedtuple
from itertools import product
import fileinput
import pprint

exec(open("../lib.py").read())

Pad = namedtuple("Pad", ["pad", "width", "height", "missing"])

num_pad = {
    "7": (0, 0), "8": (1, 0), "9": (2, 0),
    "4": (0, 1), "5": (1, 1), "6": (2, 1),
    "1": (0, 2), "2": (1, 2), "3": (2, 2),
                 "0": (1, 3), "A": (2, 3)
}
num_pad = Pad(num_pad, 3, 4, (0, 3))

dir_pad = {
                 "^": (1, 0), "A": (2, 0),
    "<": (0, 1), "v": (1, 1), ">": (2, 1)
}
dir_pad = Pad(dir_pad, 3, 2, (0, 0))

moves_x = { -1: "<", 0: "", 1: ">" }
moves_y = { -1: "^", 0: "", 1: "v" }

def move_symbol(vec):
    if vec[0] != 0:
        return moves_x[vec[0]]
    else:
        return moves_y[vec[1]]

def to_vec(frompoint, topoint):
    dx = topoint[0] - frompoint[0]
    dy = topoint[1] - frompoint[1]
    vx = int(dx / abs(dx)) if dx != 0 else dx
    vy = int(dy / abs(dy)) if dy != 0 else dy
    return (vx, vy)

def find_paths(keypad, frompoint, topoint):
    if frompoint == topoint:
        return [['A']]

    paths = []
    vx, vy = to_vec(frompoint, topoint)
    for vec in [(vx, 0), (0, vy)]:
        newpoint = (frompoint[0] + vec[0], frompoint[1] + vec[1])
        if newpoint == frompoint or newpoint == keypad.missing:
            continue
        if newpoint[0] < 0 or newpoint[0] >= keypad.width:
            continue
        if newpoint[1] < 0 or newpoint[1] >= keypad.height:
            continue
        for path in find_paths(keypad, newpoint, topoint):
            paths.append([move_symbol(vec)] + path)

    return paths

limit = 3
def enter_symbols(keypad, symbols, i):
    if i == limit:
        return symbols

    paths = [[]]
    current_point = keypad.pad['A']
    for symbol in symbols:
        new_paths = find_paths(keypad, current_point, keypad.pad[symbol])
        paths = [ oldpath + newpath for oldpath, newpath in product(paths, new_paths) ]
        current_point = keypad.pad[symbol] 

    paths = [ enter_symbols(dir_pad, path, i+1) for path in paths]
    shortest = paths[0]
    for path in paths:
        if len(path) < len(shortest):
            shortest = path
    return shortest

def main():
    codes = [line[:-1] for line in fileinput.input()]
    total = 0
    for code in codes:
        debug(f"code {code}")
        moves = enter_symbols(num_pad, code, 0)
        debug("".join(moves))
        total += int(code[:-1]) * len(moves)
    print(total)

main()
