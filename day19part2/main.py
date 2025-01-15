#!/usr/bin/env python3
import fileinput

def count_matches(pattern, towels, tmax, memory):
    if pattern in memory:
        return memory[pattern]
    indices = range(1, min(tmax+1, len(pattern)))
    total = sum(count_matches(pattern[i:], towels, tmax,  memory) if pattern[:i] in towels else 0 for i in indices)
    total += 1 if pattern in towels else 0
    memory[pattern] = total
    return total

def main():
    lines = list(fileinput.input())
    towels = set(lines[0].strip().split(", "))
    tmax = max([ len(k) for k in towels ])
    patterns = [ line.strip() for line in lines[2:] ]
    print(sum(count_matches(pattern, towels, tmax, {}) for pattern in patterns))

main()

