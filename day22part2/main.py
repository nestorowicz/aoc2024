#!/usr/bin/env python3
import fileinput

def process(n):
    n = ((n * 64) ^ n) % 16777216
    n = ((n // 32) ^ n) % 16777216
    n = ((n * 2048) ^ n) % 16777216
    return n

def main():
    nums = [ int(line.strip()) for line  in fileinput.input() ]
    scores = {}
    best = 0
    for n in nums:
        changes = []
        seen = set()
        for i in range(0, 2000):
            nn = process(n)
            price = nn % 10
            changes.append(price - (n % 10))
            n = nn
            key = tuple(changes[-4:])
            if key in seen or len(key) != 4:
                continue
            seen.add(key)
            scores[key] = scores.get(key, 0) + price
            if scores[key] > scores.get(best, 0):
                best = key

    print(scores[best])

main()
