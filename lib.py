from collections import namedtuple
import os

def debug(arg):
    if "DEBUG" in os.environ:
        print(arg)

Point = namedtuple('Point', ['x', 'y'])
DIRECTION_UP = 0
DIRECTION_RIGHT = 1
DIRECTION_DOWN = 2
DIRECTION_LEFT = 3
Ray = namedtuple('Ray', ['point', 'direction'])

def rotate_ray_right(ray):
    return Ray(ray.point, (ray.direction + 1) % 4)
def rotate_ray_left(ray):
    return Ray(ray.point, (ray.direction - 1) % 4)
def move_ray(ray):
    x = ray.point.x
    y = ray.point.y
    if ray.direction == DIRECTION_UP:
        return Ray(Point(x, y-1), DIRECTION_UP)
    if ray.direction == DIRECTION_RIGHT:
        return Ray(Point(x+1, y), DIRECTION_RIGHT)
    if ray.direction == DIRECTION_DOWN:
        return Ray(Point(x, y+1), DIRECTION_DOWN)
    if ray.direction == DIRECTION_LEFT:
        return Ray(Point(x-1, y), DIRECTION_LEFT)
    raise Exception(f"Unexpected direction {ray.direction}")

def print_board(board):
    for line in board:
        for symbol in line:
            print(symbol, end='')
        print()

def parse_board():
    return [ [ c for c in line.strip() ] for line in fileinput.input() ]

def neighbors_cross(board, pt):
    bh = len(board)
    bw = len(board[0])
    return [pt for pt in [Point(pt.x+1,pt.y), Point(pt.x-1,pt.y), Point(pt.x,pt.y+1), Point(pt.x,pt.y-1)] if (0 < pt.x < bw) and (0 < pt.y < bh)]

def find_start_end(board):
    start = Point(0, 0)
    end = Point(0, 0)
    for y, line in enumerate(board):
        for x, c in enumerate(line):
            if c == 'S':
                start = Point(x, y)
            if c == 'E':
                end = Point(x, y)
    return start, end
