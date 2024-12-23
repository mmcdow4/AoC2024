import sys
import numpy as np

def get_computer_id(all_computers, computer_name):
    for (id, name) in all_computers.items():
        if name == computer_name:
            return id
    return np.nan

def clique_to_string(clique, all_computers):
    clique_string = ""
    clique_names = []
    for id in clique:
        clique_names.append(all_computers[id])
    
    clique_names.sort()
    for name in clique_names:
        clique_string += "{},".format(name)
    
    return clique_string[0:len(clique_string)-1]

def extract_t_triplets(clique, all_computers):
    triplets = set()
    for id1 in clique:
        if all_computers[id1][0] == 't':
            for id2 in clique:
                for id3 in clique:
                    if not id1 == id2 and not id1 == id3 and not id2 == id3:
                        triplets.add(clique_to_string([id1, id2, id3], all_computers))
    return triplets

def bron_kerbosch(R, P, X, graph, cliques):
    if not P and not X:
        cliques.append(R)
        return
    
    for v in list(P):
        neighbors = set(graph[v])
        newR = R | {v}
        newP = P & neighbors
        newX = X & neighbors
        bron_kerbosch(
            newR,
            newP,
            newX,
            graph,
            cliques
        )
        P.remove(v)
        X.add(v)

input_file = sys.argv[1]

all_computers = dict()
connection_graph = dict()
id_counter = 0
with open(f"E:\\dev\\AoC2024\\day23\\{input_file}") as file:
    for line in file:
        line = line.replace("\n", "")
        computers = line.split("-")

        computer_id1 = get_computer_id(all_computers, computers[0])
        computer_id2 = get_computer_id(all_computers, computers[1])
        if np.isnan(computer_id1):
            computer_id1 = id_counter
            all_computers[computer_id1] = computers[0]
            id_counter += 1
        if np.isnan(computer_id2):
            computer_id2 = id_counter
            all_computers[computer_id2] = computers[1]
            id_counter += 1

        if computer_id1 in connection_graph:
            connection_graph[computer_id1].append(computer_id2)
        else:
            connection_graph.update({computer_id1: [computer_id2]})
            
        if computer_id2 in connection_graph:
            connection_graph[computer_id2].append(computer_id1)
        else:
            connection_graph.update({computer_id2: [computer_id1]})


cliques = []
bron_kerbosch(set(), set(connection_graph.keys()), set(), connection_graph, cliques)

print(f"Found {len(cliques)} cliques")
largest_network = clique_to_string(max(cliques, key=len), all_computers)

triplets = set()
for clique in cliques:
    new_triplets = extract_t_triplets(clique, all_computers)
    for triplet in new_triplets:
        triplets.add(triplet)

print(f"Found {len(triplets)} triplet connections")
print(f"Longest network is {largest_network}")