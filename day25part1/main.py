#!/usr/bin/env python3
import fileinput

def parse(lines):
    return [ sum(1 for c in col if c == '#') for col in list(zip(*lines))]

def overlaps(key, lock):
    return next(iter(filter(lambda t: t[0] + t[1] > 5, zip(key, lock))), None)

def main():
    lines = [ line.rstrip() for line in fileinput.input() ]
    locks = [ parse(lines[i+1:i+6]) for i in range(0, len(lines), 8) if lines[i][0] == '#' ]
    keys = [ parse(lines[i+1:i+6]) for i in range(0, len(lines), 8) if lines[i][0] != '#' ]
    matches = sum([ 1 for key in keys for lock in locks if not overlaps(key, lock) ])
    print(matches)

main()
