#!/usr/bin/env python3
import fileinput

def findgroups(graph, group, limit):
    if len(group) == limit:
        return [tuple(sorted(group))]
    groups = []
    for child in graph[group[-1]]:
        if child in group:
            continue
        if len([ node for node in group if node in graph[child] ]) != len(group):
            continue
        nextgroup = group.copy()
        nextgroup.append(child)
        groups.extend(findgroups(graph, nextgroup, limit))
    return groups

def main():
    conns = [ tuple(line.strip().split('-')) for line in fileinput.input() ]
    graph = {}
    for a, b in conns:
        ac = graph.get(a, set())
        ac.add(b)
        graph[a] = ac

        bc = graph.get(b, set())
        bc.add(a)
        graph[b] = bc

    groups = []
    for node in graph:
        if node[0] != 't':
            continue
        groups.extend(findgroups(graph, [node], 3))
    print(len(list(set(groups))))

main()
