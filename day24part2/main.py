#!/usr/bin/env python3
import fileinput
import itertools
from collections import namedtuple

Gate = namedtuple('Gate', ['a', 'op', 'b', 'out'])

def parse_gate(line):
    inp, out = tuple(line.split(' -> '))
    a, op, b = tuple(inp.split(' '))
    return Gate(a, op, b, out)

def build_gates_by_in(gates):
    gates_by_in = {}
    for gate in gates:
        sub = gates_by_in.get(gate.a, [])
        sub.append(gate)
        gates_by_in[gate.a] = sub

        sub = gates_by_in.get(gate.b, [])
        sub.append(gate)
        gates_by_in[gate.b] = sub
    return gates_by_in

# gates are a map of gates by input wires
# first returned value is the name of the carry wire
# half adder consists of: 
#   x XOR y = sum (z)
#   x AND y = carry
def validate_half_adder(gates, x, y, z):
    if gates[x] != gates[y]:
        raise Exception('invalid half adder')
    if len(gates[x]) != 2 or gates[x] != gates[y]:
        raise Exception('invalid half adder')

    gcarry = gates[x][0]
    if gcarry.op != 'AND':
        raise Exception('invalid half adder')

    gsum = gates[x][1]
    if gsum.op != 'XOR' or gsum.out != z:
        raise Exception('invalid half adder')

    return gcarry.out

# gates are a map of gates by input wires
# first returned value is the name of the carry wire
# adder consists of: 
#   x XOR y = a
#   x AND y = b
#   a XOR c = sum (z)
#   a AND c = d
#   b OR d = carry
def validate_adder(gates, i, c):
    x = 'x%02d' % i
    y = 'y%02d' % i
    z = 'z%02d' % i
    if len(gates[x]) != 2 or gates[x] != gates[y]:
        raise Exception('invalid adder')

    print(f"gates[x] {gates[x]} carry {c}")

    ga = gates[x][1]
    if ga.op != 'XOR':
        raise Exception('invalid adder')
    a = ga.out

    print(f"gates[a] {gates[a]}")
    gb = gates[x][0]
    if gb.op != 'AND':
        raise Exception('invalid adder')
    b = gb.out
    print(f"gates[b] {gates[b]}")

    gz = gates[a][1]
    if gz.op != 'XOR':
        raise Exception('invalid adder')
    if gz.out[0] != 'z':
        raise Exception('invalid adder')

    if len(gates[a]) != 2:
        raise Exception('invalid adder')
    
    print()
    return ''


def main():
    lines = list(itertools.dropwhile(lambda e: e != '\n', fileinput.input()))[1:]
    gates = [ parse_gate(ln.rstrip()) for ln in lines ]
    swaps = {
        'vvm': 'dgr',
        'dgr': 'vvm',

        'z37': 'dtv',
        'dtv': 'z37',

        'z12': 'fgc',
        'fgc': 'z12',

        'z29': 'mtj',
        'mtj': 'z29'
    }
    gates = [ Gate(g.a, g.op, g.b, swaps.get(g.out, g.out)) for g in gates ]
    gates_by_in = { wire: sorted(gates, key=lambda g: g.op) for wire, gates in build_gates_by_in(gates).items()}

    carry = validate_half_adder(gates_by_in, 'x00', 'y00', 'z00')
    for i in range(1, 45):
        carry = validate_adder(gates_by_in, i, carry)

    print(",".join(sorted([s for s in swaps])))

main()
