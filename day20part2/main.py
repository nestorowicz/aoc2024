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

DMAX = 20
def find_neighbors_with_cheats(board, pt):
    for dx in range(-DMAX, DMAX+1):
        for dy in range(-DMAX+abs(dx), DMAX+1-abs(dx)):
            yield (Point(pt.x+dx, pt.y+dy), abs(dx)+abs(dy))

def find_cheats(board, start, end, distances):
    cheats = {}
    total_distance = distances[end]
    for pt in distances: # point
        for nb, dist in find_neighbors_with_cheats(board, pt):
            if nb == pt:
                continue
            if not is_on_board(nb, board):
                continue
            if board[nb.y][nb.x] not in ['.','E']:
                continue
            if (pt, nb) in cheats:
                continue
            new_nb_distance = distances[pt] + dist
            if new_nb_distance >= distances[nb]:
                continue
            cheats[(pt, nb)] = distances[nb] - new_nb_distance
    return cheats

def main():
    board=parse_board()
    start, end = find_start_end(board)
    distances = calculate_distances(board, start, end)
    cheats = find_cheats(board, start, end, distances)
    total = sum([ 1 for cheat, saved in cheats.items() if saved >= 100 ])
    print(total)

main()
