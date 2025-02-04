#!/usr/bin/env python3
from collections import namedtuple
import fileinput

exec(open("../lib.py").read())

Pad = namedtuple("Pad", ["pad", "index"])

NUM_PAD = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3'], ['', '0', 'A']]
NUM_PAD = Pad(NUM_PAD, { s: (x, y) for y, row in enumerate(NUM_PAD) for x, s in enumerate(row) })

DIR_PAD = [['', '^', 'A'], ['<', 'v', '>']]
DIR_PAD = Pad(DIR_PAD, { s: (x, y) for y, row in enumerate(DIR_PAD) for x, s in enumerate(row) })

MOVES = {(-1, 0):"<", (0, 1):"v", (1, 0):">", (0, -1):"^"}

def find_paths(keypad, frompoint, topoint):
    if frompoint == topoint:
        return [['A']]

    paths = []
    vx = -1 if topoint[0] < frompoint[0] else 1 if topoint[0] > frompoint[0] else 0
    vy = -1 if topoint[1] < frompoint[1] else 1 if topoint[1] > frompoint[1] else 0
    for vec in [(vx, 0), (0, vy)]:
        x, y = (frompoint[0] + vec[0], frompoint[1] + vec[1])
        if vec == (0,0) or 0 > y >= len(keypad.pad) or 0 > x >= len(keypad.pad[y]) or keypad.pad[y][x] == '':
            continue
        paths.extend([ [MOVES[vec]] + subpath for subpath in find_paths(keypad, (x, y), topoint) ])

    return paths

cache = {}
def find_shortest_path_cached(keypad, fromsymbol, tosymbol, i):
    key = (fromsymbol, tosymbol, i)
    return cache[key] if key in cache else cache.setdefault(key, find_shortest_path(keypad, fromsymbol, tosymbol, i))

def find_shortest_path(keypad, fromsymbol, tosymbol, i):
    paths = find_paths(keypad, keypad.index[fromsymbol], keypad.index[tosymbol])
    return min([ enter_symbols(DIR_PAD, path, i+1) for path in paths ])

limit = 26
def enter_symbols(keypad, symbols, depth):
    if depth == limit:
        return len(symbols)

    s = ['A'] + list(symbols)
    return sum( find_shortest_path_cached(keypad, s[i], s[i+1], depth) for i in range(0, len(s)-1) )

def main():
    codes = [line[:-1] for line in fileinput.input()]
    total = sum([ int(code[:-1]) * enter_symbols(NUM_PAD, code, 0) for code in codes ])
    print(total)

main()
