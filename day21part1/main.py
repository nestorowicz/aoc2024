#!/usr/bin/env python3
import fileinput

exec(open("../lib.py").read())

num_pad = {
    "7": (0, 0), "8": (1, 0), "9": (2, 0),
    "4": (0, 1), "5": (1, 1), "6": (2, 1),
    "1": (0, 2), "2": (1, 2), "3": (2, 2),
                 "0": (1, 3), "A": (2, 3)
}
dir_pad = {
                 "^": (1, 0), "A": (2, 0),
    "<": (0, 1), "v": (1, 1), ">": (2, 1)
}
moves_x = { -1: "<", 0: "", 1: ">" }
moves_y = { -1: "^", 0: "", 1: "v" }
robot_limit = 3

def new_robots():
    return [
        (num_pad, num_pad['A']),
        (dir_pad, dir_pad['A']),
        (dir_pad, dir_pad['A'])
    ]

def find_shortest_path(keypad, frompoint, topoint):
    dx = topoint[0] - frompoint[0]
    dy = topoint[1] - frompoint[1]
    if dx == 0 and dy == 0:
        return ['A']
    vx = int(dx / abs(dx)) if dx != 0 else dx
    vy = int(dy / abs(dy)) if dy != 0 else dy
    mx = [moves_x[vx]] * abs(dx)
    my = [moves_y[vy]] * abs(dy)
    moves = mx + my
    if not any(gotpoint == (frompoint[0] + dx, frompoint[1]) for symbol, gotpoint in keypad.items())
        moves = my + mx
    moves.append('A')
    return moves

def to_vec(frompoint, topoint):
    dx = topoint[0] - frompoint[0]
    dy = topoint[1] - frompoint[1]
    vx = int(dx / abs(dx)) if dx != 0 else dx
    vy = int(dy / abs(dy)) if dy != 0 else dy
    return (vx, vy)

def find_cheapest_path(frompoint, topoint):
    

def enter_symbol(point, keypad, i):
    if i == robot_limit:
        return [point]
    frompoint = robots[i][1]
    vx, vy = to_vec(frompoint, topoint)
    result = []
    while vx != 0 || vy != 0:
        if vx != 0 && vy != 0:
            result_x = enter_symbol((frompoint[0]+vx, frompoint[1]), robots.deepcopy())
            result_y = enter_symbol((frompoint[0], frompoint[1]+vy), robots.deepcopy())
    return result

def keypad_contains(keypad, point):
    return any(gotpoint == (point[0], point[1]) for symbol, gotpoint in keypad.items())

def find_paths(keypad, frompoint, topoint):
    if frompoint == topoint:
        return ['A']

    paths = []
    vx, vy = to_vec(frompoint, topoint)
    if vx != 0 && keypad_contains(keypad, pointx):
        pointx = (frompoint[0]+vx, frompoint[1])
        for path in find_paths(pointx, topoint):
            paths.append([moves_x[vx]] + path)

    if vy != 0 && keypad_contains(keypad, pointy):
        pointy = (frompoint[0], frompoint[1]+vy)
        for path in find_paths(pointy, topoint):
            paths.append([moves_y[vy]] + path)

    return paths

def find_cheapest_path(keypad, paths):
    min_path = []
    min_distance = 0
    for path in paths:
        if len(path) < 2:
            return path
        prev_symbol = path[0]
        prev_point = keypad[prev_symbol]
        distance = 0
        for i, symbol in enumerate(path[1:]):
            point = keypad[symbol]
            distance += abs(point[0]-prev_point[0]) + abs(point[1]-prev_point[1])
            prev_point = point
        if min_distance == 0 or distance < min_distance:
            min_distance = distance
            min_path = path
    return path

def enter_symbols(symbols, keypad, i):
    current_position = keypad['A']
    for symbol in symbols:

    path = find_cheapest_path(find_paths(keypad), keypad)
    return moves

def main():
    codes = [line[:-1] for line in fileinput.input()]
    total = 0
    for code in codes:
        debug(f"code {code}")
        moves = enter_symbols(code, numpad, 0)
        debug("\n" + "".join(moves))
        total += int(code[:-1]) * len(moves)
        debug(f"adding to total")
    print(total)

main()
