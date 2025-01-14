#!/usr/bin/env python3
from collections import namedtuple
import fileinput
import heapq

exec(open("../lib.py").read())

BOARD_WIDTH=71
BOARD_HEIGHT=71

def find_shortest_paths(corrupted):
    start = Point(0, 0)
    unvisited = []
    heapq.heappush(unvisited, (0, start))
    distances = {start: 0}
    paths = {start: []}
    while len(unvisited) > 0:
        current_point = heapq.heappop(unvisited)[1]
        if current_point == Point(BOARD_WIDTH-1,BOARD_HEIGHT-1):
            break
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

def parse_line(line):
    line = line.strip().split(',')
    return Point(int(line[0]), int(line[1]))

def main():
    corrupted = [ parse_line(line) for line in fileinput.input() ]
    path_found = True
    byte_no = 1
    while path_found:
        paths = find_shortest_paths(corrupted[:byte_no])
        path_found = Point(BOARD_WIDTH-1,BOARD_HEIGHT-1) in paths
        byte_no += 1
    print(corrupted[byte_no-2])

main()
