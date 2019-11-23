use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::vec::Vec;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    names: Vec<String>,
    partner_map: HashMap<String, String>,
}

enum Edge {
    Linked,
    Unlinked,
}

fn main() {
    let config = read_config();
    let mut graph = build_graph(&config);
    let mut assignments: HashMap<String, String> = HashMap::new();

    for i in 0..graph.len() {
        let linked_edge_idxs = get_linked_edges(&graph[i]);

        if linked_edge_idxs.len() > 0 {
            // We have liked edges for this node
            // Select one person at random
            let idx = select_random_edge_idx(&linked_edge_idxs);
            // Assign the selected person to the current person
            assignments.insert(config.names[i].clone(), config.names[idx].clone());
            // Remove all remaining incoming links to the selected person
            // so they may not be selected again
            for j in i..graph.len() {
                graph[j][idx] = Edge::Unlinked;
            }
        } else {
            // There are no linked edges for this node,
            // meaning the last person can only select themself
            // Find someone to swap while keeping all assignments valid
            let swap_key = find_valid_swap(&assignments, &config, i);
            let swap_value = get_map_val(&assignments, &swap_key);
            assignments.insert(swap_key.clone(), config.names[i].clone());
            assignments.insert(config.names[i].clone(), swap_value.clone());
        }
    }

    let s = serde_json::to_string_pretty(&assignments).unwrap();
    println!("{}", s);
}

fn read_config() -> Config {
    let file = File::open("config.json").expect("unable to open config file");
    serde_json::from_reader(file).expect("unable to parse config file")
}

fn build_graph(config: &Config) -> Vec<Vec<Edge>> {
    (0..config.names.len())
        .collect::<Vec<usize>>()
        .iter()
        .map(|&i| {
            (0..config.names.len())
                .collect::<Vec<usize>>()
                .iter()
                .map(|&j| {
                    let partner = get_map_val(&config.partner_map, &config.names[i]);
                    let is_same_person = config.names[i] == config.names[j];
                    let is_partner = partner == config.names[j];
                    match is_same_person || is_partner {
                        true => Edge::Unlinked,
                        false => Edge::Linked,
                    }
                })
                .collect::<Vec<Edge>>()
        })
        .collect::<Vec<Vec<Edge>>>()
}

fn get_linked_edges(edges: &Vec<Edge>) -> Vec<usize> {
    (0..edges.len())
        .collect::<Vec<usize>>()
        .iter()
        .filter(|&i| match edges[*i] {
            Edge::Linked => true,
            Edge::Unlinked => false,
        })
        .map(|&i| i)
        .collect::<Vec<usize>>()
}

fn select_random_edge_idx(linked_edge_idxs: &Vec<usize>) -> usize {
    let n = rand::thread_rng().gen_range(0, linked_edge_idxs.len());
    linked_edge_idxs[n]
}

fn find_valid_swap(pairs: &HashMap<String, String>, config: &Config, i: usize) -> String {
    let partner = get_map_val(&config.partner_map, &config.names[i]);
    match &partner[..] {
        // This person has no partner
        // Pick someone else at random to swap with
        "" => {
            let n = rand::thread_rng().gen_range(0, config.names.len() - 1);
            config.names[n].clone()
        }
        // Swap with whoever this person's partner was assigned
        _ => get_map_val(&pairs, &partner),
    }
}

fn get_map_val(map: &HashMap<String, String>, key: &str) -> String {
    String::from(match map.get(key) {
        Some(n) => n,
        None => "",
    })
}
