#!/usr/bin/env python3
import fileinput

def is_connected(graph, node, others):
    return next((False for other in others if other not in graph[node]), True)

def findlongest(graph, group, visited):
    if tuple(group) in visited:
        return visited[tuple(group)]
    best = ()
    for child in graph[group[-1]]:
        if child in group:
            continue
        if not is_connected(graph, child, group):
            continue
        nextgroup = group.copy()
        nextgroup.append(child)
        newbest = findlongest(graph, sorted(nextgroup), visited)
        if len(newbest) > len(best):
            best = newbest
    if best == ():
        return tuple(group)
    visited[tuple(group)] = best
    return best

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

    visited = {}
    longest = ()
    for node in graph:
        if node[0] != 't':
            continue
        n = findlongest(graph, [node], visited)
        if len(n) > len(longest):
            longest = n
    print(','.join(sorted(list(longest))))

main()
