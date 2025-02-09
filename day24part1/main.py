#!/usr/bin/env python3
import fileinput
from itertools import takewhile
from collections import namedtuple

Gate = namedtuple('Gate', ['a', 'op', 'b', 'out'])

def parse_gate(line):
    inp, out = tuple(line.split(' -> '))
    a, op, b = tuple(inp.split(' '))
    return Gate(a, op, b, out)

def calculate(a, op, b):
    if op == 'AND':
        return a & b
    if op == 'OR':
        return a | b
    if op == 'XOR':
        return a ^ b
    raise Exception('huh')

def find_num(states, prefix):
    out = sorted([ s for s in states if s[0] == prefix ])
    return sum(pow(2, i) * states[out[i]] for i in range(0, len(out)) )

def find_solution(init_states, gates):
    queue = [ node for node in gates if node[0] == 'z' ]
    states = { k: v for k, v in init_states.items() }

    while True:
        if len(queue) == 0:
            break
        node = queue.pop()
        if node in states:
            continue
        a, op, b, out = gates[node]
        if a in states and b in states:
            states[node] = calculate(states[a], op, states[b])
            pass
        queue.append(node)
        if a not in states:
            queue.append(a)
        if b not in states:
            queue.append(b)

    return find_num(states, 'z')

def main():
    finput = fileinput.input()
    lines = [ tuple(ln.rstrip().split(': ')) for ln in takewhile(lambda e: e != '\n', finput) ]
    init_states = { k: int(v) for k, v in lines }
    lines = [ parse_gate(ln.rstrip()) for ln in finput ]
    gates = { gate.out: gate for gate in lines }
    print(find_solution(init_states, gates))

main()
