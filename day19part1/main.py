#!/usr/bin/env python3
import fileinput
from itertools import groupby

exec(open("../lib.py").read())

def is_possible(pattern, towelsmap):
    for t in towelsmap.get(pattern[0], []):
        if pattern.startswith(t) and ( len(t) == len(pattern) or is_possible(pattern[len(t):], towelsmap)):
            return True
    return False

def main():
    lines = list(fileinput.input())
    towels = { k: list(g) for k, g in groupby(sorted(lines[0].strip().split(", ")), lambda x: x[0])}
    patterns = [ line.strip() for line in lines[2:] ]
    possible = sum([ 1 for pattern in patterns if is_possible(pattern, towels)])
    print(possible)

main()

