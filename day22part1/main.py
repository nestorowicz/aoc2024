#!/usr/bin/env python3
import fileinput

def process(n):
    a = n * 64
    n = a ^ n
    n = n % 16777216
    b = (n // 32)
    n = b ^ n
    n = n % 16777216
    c = (n * 2048)
    n = c ^ n
    n = n % 16777216
    return n

def main():
    nums = [ int(line.strip()) for line  in fileinput.input() ]
    total = 0
    for n in nums:
        for i in range(0, 2000):
            n = process(n)
        total += n
    print(total)

main()
