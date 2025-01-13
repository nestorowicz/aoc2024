#!/usr/bin/env python3
from collections import namedtuple
import fileinput
import heapq

exec(open("../lib.py").read())

def find_shortest_paths(board, start, end):
    unvisited = []
    heapq.heappush(unvisited, (0, start))
    distances = {start: 0}
    paths = {start: []}
    while len(unvisited) > 0:
        current_ray = heapq.heappop(unvisited)[1]
        for next_ray in [move_ray(current_ray), move_ray(rotate_ray_right(current_ray)), move_ray(rotate_ray_left(current_ray))]:
            distance = distances[current_ray] + (1 if current_ray.direction == next_ray.direction else 1001)
            prev_distance = distances.get(next_ray, None)
            if not (0 <= next_ray.point.y < len(board)) or not (0 <= next_ray.point.x < len(board[next_ray.point.y])):
                continue
            if board[next_ray.point.y][next_ray.point.x] == '#':
                continue
            if prev_distance is not None and distance > prev_distance:
                continue
            if next_ray not in distances:
                heapq.heappush(unvisited, (distance, next_ray))
            if next_ray not in distances or distance < distances[next_ray]:
                distances[next_ray] = distance
                paths[next_ray] = []
            if prev_distance is None or prev_distance == distance:
                paths[next_ray].append(current_ray)
    return paths

def map_paths_to_points(paths, end):
    rays = []
    queue = [ ray for ray in paths if ray.point == end ]
    while len(queue) > 0:
        rays += [ ray for ray in queue ]
        queue = [ prev for ray in queue for prev in paths[ray] ]
    return set([ ray.point for ray in rays ])

def main():
    board = [[ c for c in line ] for line in fileinput.input()]
    start = next(Ray(Point(x,y), DIRECTION_RIGHT) for y, line in enumerate(board) for x, c in enumerate(line) if c == 'S')
    end = next(Point(x,y) for y, line in enumerate(board) for x, c in enumerate(line) if c == 'E')
    paths = find_shortest_paths(board, start, end)
    points = map_paths_to_points(paths, end)
    print(len(points))

main()
