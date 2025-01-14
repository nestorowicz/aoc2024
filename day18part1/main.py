#!/usr/bin/env python3
from collections import namedtuple
import fileinput
import heapq

exec(open("../lib.py").read())

BOARD_WIDTH=71
BOARD_HEIGHT=71
LIMIT=1024

def find_shortest_paths(corrupted):
    start = Point(0, 0)
    unvisited = []
    heapq.heappush(unvisited, (0, start))
    distances = {start: 0}
    paths = {start: []}
    while len(unvisited) > 0:
        current_point = heapq.heappop(unvisited)[1]
        for next_point in [
                Point(current_point.x+1,current_point.y),
                Point(current_point.x-1,current_point.y),
                Point(current_point.x,current_point.y+1),
                Point(current_point.x,current_point.y-1)
            ]:
            distance = distances[current_point] + 1
            prev_distance = distances.get(next_point, None)
            if not (0 <= next_point.y < BOARD_HEIGHT) or not (0 <= next_point.x < BOARD_WIDTH):
                continue
            if next_point in corrupted:
                continue
            if prev_distance is not None and distance > prev_distance:
                continue
            if next_point not in distances:
                heapq.heappush(unvisited, (distance, next_point))
            if next_point not in distances or distance < distances[next_point]:
                distances[next_point] = distance
                paths[next_point] = []
            if prev_distance is None:
                paths[next_point].append(current_point)
    return paths

def map_paths_to_points(paths, end):
    points = []
    queue = [ point for point in paths if point == end ]
    while len(queue) > 0:
        points += [ point for point in queue ]
        queue = [ prev for point in queue for prev in paths[point] ]
    return set(points)

def parse_line(line):
    line = line.strip().split(',')
    return Point(int(line[0]), int(line[1]))

def main():
    corrupted = [ parse_line(line) for line in fileinput.input() ][:LIMIT]
    paths = find_shortest_paths(corrupted)
    points = map_paths_to_points(paths, Point(BOARD_WIDTH-1, BOARD_HEIGHT-1))
    board = [['.'] * BOARD_WIDTH for i in range(BOARD_HEIGHT)]
    for c in corrupted:
        board[c.y][c.x] = '#'
    for p in points:
        board[p.y][p.x] = 'O'
    print_board(board)
    print(len(points)-1)

main()
