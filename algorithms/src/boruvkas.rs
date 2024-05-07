use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Edge {
    start: usize,
    end: usize,
    weight: usize,
}

struct Graph {
    edges: Vec<Edge>,
    vertices: usize,
}

fn boruvkas_algorithm(graph: &Graph) -> Vec<Edge> {
    let mut mst: Vec<Edge> = Vec::new();
    let mut components: HashMap<usize, HashSet<usize>> = HashMap::new();

    for v in 0..graph.vertices {
        let mut set = HashSet::new();
        set.insert(v);
        components.insert(v, set);
    }

    while mst.len() < graph.vertices - 1 {
        let mut cheapest_edge: HashMap<usize, Option<Edge>> = HashMap::new();

        for (component, vertices) in components.iter() {
            cheapest_edge.insert(*component, None);

            for edge in graph.edges.iter() {
                if vertices.contains(&edge.start) && !vertices.contains(&edge.end)
                    || vertices.contains(&edge.end) && !vertices.contains(&edge.start)
                {
                    match cheapest_edge.get_mut(component) {
                        Some(cheapest) => {
                            if let Some(cur_cheapest) = cheapest {
                                if edge.weight < cur_cheapest.weight {
                                    *cheapest = Some(edge.clone());
                                }
                            } else {
                                *cheapest = Some(edge.clone());
                            }
                        }
                        None => {
                            cheapest_edge.insert(*component, Some(edge.clone()));
                        }
                    }
                }
            }
        }

        for (_, edge) in cheapest_edge.iter() {
            if let Some(e) = edge {
                let start_component = components.get(&e.start).unwrap().clone();
                let end_component = components.get(&e.end).unwrap().clone();

                if start_component != end_component {
                    mst.push(e.clone());

                    let merged_component: HashSet<usize> =
                        start_component.union(&end_component).cloned().collect();
                    for vertex in merged_component.iter() {
                        components.insert(*vertex, merged_component.clone());
                    }
                }
            }
        }
    }

    mst
}
