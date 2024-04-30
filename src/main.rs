use petgraph::Graph;
use petgraph::Undirected;
use std::collections::{HashMap, HashSet};
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Anime {
    title: String,
    genre: String, 
}

fn main() {
    let mut graph = Graph::<String, usize, Undirected>::new_undirected();
    let mut reader = Reader::from_path("processed_animes.csv").expect("Cannot read file");

    let mut genre_map: HashMap<String, HashSet<String>> = HashMap::new();

    
    for result in reader.deserialize() {
        let anime: Anime = result.expect("Error parsing CSV");
        let genres: HashSet<String> = anime.genre.split(',')
                                                .map(|s| s.trim().to_string())
                                                .collect();
        genre_map.insert(anime.title.clone(), genres);

        
        graph.add_node(anime.title.clone());
    }

    
    for (title1, genres1) in &genre_map {
        for (title2, genres2) in &genre_map {
            if title1 != title2 {
                let shared_genres = genres1.intersection(genres2).count();
                if shared_genres > 0 {
                    graph.add_edge(title1.clone(), title2.clone(), shared_genres);
                }
            }
        }
    }

    println!("Graph has {} nodes and {} edges.", graph.node_count(), graph.edge_count());

    
    if graph.node_count() >= 1000 && graph.edge_count() >= 1000 {
        println!("Graph meets the required criteria!");
    } else {
        println!("Graph does not meet the required criteria, consider adjusting the dataset or edge criteria.");
    }
}
