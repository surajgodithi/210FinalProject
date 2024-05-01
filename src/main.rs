use csv::{ReaderBuilder, Trim};
use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug, Deserialize)]
struct Anime {
    title: String,
    genre: String,
    score: f32,
}

fn main() {
    let mut graph = Graph::<(String, HashSet<String>, f32), usize, Undirected>::new_undirected();
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .trim(Trim::All)
        .from_path("processed_animes.csv")
        .expect("Cannot read file");

    let mut all_nodes = vec![];

    for result in reader.deserialize::<Anime>() {
        match result {
            Ok(anime) => {
                let genres: HashSet<String> = anime.genre.split(',')
                                                        .map(|s| s.trim().to_string())
                                                        .collect();
                all_nodes.push((anime.title, genres, anime.score));
            },
            Err(e) => {
                eprintln!("Error parsing CSV: {:?}", e);
                continue;
            }
        }
    }

    let mut rng = thread_rng();
    all_nodes.shuffle(&mut rng);
    let selected_nodes = &all_nodes[..1000.min(all_nodes.len())];

    let mut index_map: HashMap<String, NodeIndex> = HashMap::new();

    for (title, genres, score) in selected_nodes {
        let node_index = graph.add_node((title.clone(), genres.clone(), *score));
        index_map.insert(title.clone(), node_index);
    }

    let mut potential_edges = Vec::new();
    let node_data: Vec<(NodeIndex, HashSet<String>)> = index_map.iter().map(|(_, &node)| (node, graph[node].1.clone())).collect();

    for (node1, genres1) in &node_data {
        for (node2, genres2) in &node_data {
            if *node1 != *node2 && genres1.intersection(genres2).count() > 0 {
                potential_edges.push((*node1, *node2));
            }
        }
    }

    for (node1, node2) in potential_edges {
        graph.add_edge(node1, node2, 1);
    }

    let influence_score = calculate_influence_score(&graph);
    let mut influence_score_vec: Vec<_> = influence_score.into_iter().collect();
    influence_score_vec.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top 10 Most Influential Animes:");
    for (title, degree) in influence_score_vec.iter().take(10) {
        println!("Anime: {:?}, Influence Score: {}", title, degree);
    }

}

fn calculate_influence_score(graph: &Graph<(String, HashSet<String>, f32), usize, Undirected>) -> HashMap<String, usize> {
    graph.node_indices().map(|n| (graph[n].0.clone(), graph.neighbors(n).count())).collect()
}

#[test]
fn test_influence_score_calculation() {
    let mut graph = Graph::<(String, HashSet<String>, f32), usize, Undirected>::new_undirected();
    let node_index1 = graph.add_node(("Naruto".to_string(), ["Action", "Adventure"].iter().map(|&s| s.to_string()).collect(), 8.5));
    let node_index2 = graph.add_node(("Bleach".to_string(), ["Action", "Drama"].iter().map(|&s| s.to_string()).collect(), 9.0));
    graph.add_edge(node_index1, node_index2, 1);

    let influence_score = calculate_influence_score(&graph);
    assert_eq!(influence_score["Naruto"], 1);
    assert_eq!(influence_score["Bleach"], 1);
}

#[test]
fn test_graph_creation() {
    let mut graph = Graph::<(String, HashSet<String>, f32), usize, Undirected>::new_undirected();
    let node_index1 = graph.add_node(("Naruto".to_string(), ["Action"].iter().map(|&s| s.to_string()).collect(), 8.5));
    let node_index2 = graph.add_node(("Bleach".to_string(), ["Action"].iter().map(|&s| s.to_string()).collect(), 9.0));
    graph.add_edge(node_index1, node_index2, 1);

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
}