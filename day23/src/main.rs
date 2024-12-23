use std::{fs, env, collections::{HashMap, HashSet}, cmp::Ordering};

fn bron_kerbosch(
    current_clique: &HashSet<usize>,
    candidates: &mut HashSet<usize>,
    tested: &mut HashSet<usize>,
    graph: &HashMap<usize, HashSet<usize>>,
    cliques: &mut Vec<HashSet<usize>>
) {
    if candidates.is_empty() && tested.is_empty() {
        cliques.push(current_clique.clone());
        return;
    }

    let candidates_copy = candidates.clone();
    for computer in candidates_copy {
        let mut new_clique = current_clique.clone();
        let neighbors = graph[&computer].clone();
        new_clique.insert(computer);

        let mut new_candidates = candidates.clone()
            .intersection(&neighbors)
            .map(|k| *k)
            .collect();
        let mut new_tested = tested.clone()
            .intersection(&neighbors)
            .map(|k| *k)
            .collect();

        bron_kerbosch(
            &new_clique,
            &mut new_candidates,
            &mut new_tested,
            graph,
            cliques);

        candidates.remove(&computer);
        tested.insert(computer);
    }
}

fn get_computer_id(all_computers: &HashMap<usize, String>, computer_name: &str, id_counter: usize) -> usize {
    for (tmp_id, name) in all_computers {
        if name == computer_name {
            return *tmp_id;
        }    
    }
    return id_counter;
}

fn clique_to_string(clique: &HashSet<usize>, all_computers: &HashMap<usize, String>) -> String {
    let mut names = Vec::with_capacity(clique.len());
    for computer in clique {
        names.push(all_computers[computer].clone());
    }
    names.sort();
    let mut clique_string = String::new();
    for name in names {
        clique_string += &format!("{name},");
    }
    clique_string = clique_string[0..(clique_string.len()-1)].to_string();
    clique_string
}

fn extract_t_triplets(clique: &HashSet<usize>, all_computers: &HashMap<usize, String>) -> HashSet<String> {
    let mut triplets = HashSet::new();

    for id1 in clique {
        if all_computers[id1].starts_with("t") {
            for id2 in clique {
                for id3 in clique {
                    if id1 != id2 && id1 != id3 && id2 != id3 {
                        triplets.insert(clique_to_string(&HashSet::from([*id1, *id2, *id3]), all_computers));
                    }
                }
            }
        }
    }
    triplets
}
fn main() {
    let argslist: Vec<String> = env::args().collect();
    let input_file = &argslist[1];

    let mut all_computers: HashMap<usize, String> = HashMap::new();
    let mut connection_graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut id_counter = 0;
    for line in fs::read_to_string(&format!("E:\\dev\\AoC2024\\day23\\{input_file}")).unwrap().lines() {
        let computers: Vec<&str> = line.split("-").collect();
        let (computer1, computer2) = (computers[0], computers[1]);

        let computer1_id = get_computer_id(&all_computers, computer1, id_counter);
        if computer1_id == id_counter {
            all_computers.insert(computer1_id, computer1.to_string());
            id_counter +=1;
        }
        let computer2_id = get_computer_id(&all_computers, computer2, id_counter);
        if computer2_id == id_counter {
            all_computers.insert(computer2_id, computer2.to_string());
            id_counter +=1;
        }
        connection_graph
            .entry(computer1_id)
            .or_insert(HashSet::new())
            .insert(computer2_id);
        connection_graph
            .entry(computer2_id)
            .or_insert(HashSet::new())
            .insert(computer1_id);

    }

    let mut cliques = Vec::new();
    bron_kerbosch(
        &HashSet::new(),
        &mut all_computers.keys().copied().collect(),
        &mut HashSet::new(),
        &connection_graph,
        &mut cliques
    );
    
    println!("Found {} cliques", cliques.len());
    /* Part 2 */
    cliques.sort_by(|a, b| {
        if a.len() > b.len() {
            Ordering::Less
        } else if a.len() < b.len() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    let largest_clique = clique_to_string(&cliques[0], &all_computers);
    let mut triplets = HashSet::new();
    for clique in cliques {
        if clique.len() >= 3 {
            triplets.extend(extract_t_triplets(&clique, &all_computers));
        }
    }

    println!("Found {} possible triplet connections", triplets.len());

    println!("Longest network is {largest_clique}");
}
