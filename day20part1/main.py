#!/usr/bin/env python3
import fileinput

exec(open("../lib.py").read())

def calculate_distances(board, start, end):
    distances = {}
    cp = start # current_point
    dist = 0
    while True:
        distances[cp] = dist
        dist += 1
        if cp == end:
            break
        cp = [ pt for pt in neighbors_cross(board, cp) if pt not in distances and board[pt.y][pt.x] != '#' ][0]
    return distances

def find_cheats(board, start, end, distances):
    cheats = {}
    total_distance = distances[end]
    for pt in distances: # point
        for nb1 in neighbors_cross(board, pt): # neighbor1
            for nb2 in neighbors_cross(board, nb1): # neighbor2
                if (nb1, nb2) in cheats:
                    continue
                if board[nb1.y][nb1.x] != '#':
                    continue
                if board[nb2.y][nb2.x] not in ['.','E']:
                    continue
                new_nb2_distance = distances[pt] + 2
                if new_nb2_distance >= distances[nb2]:
                    continue
                cheats[(nb1, nb2)] = distances[nb2] - new_nb2_distance
    return cheats

def main():
    board=parse_board()
    start, end = find_start_end(board)
    distances = calculate_distances(board, start, end)
    cheats = find_cheats(board, start, end, distances)
    total = sum([ 1 for cheat, saved in cheats.items() if saved >= 100 ])
    print(total)

main()
