use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io;

type Graph = HashMap<usize, Vec<(usize, usize)>>;

#[derive(Debug, PartialEq, Eq)]
struct Edge {
    node: usize,
    weight: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Graph, start: usize) -> HashMap<usize, usize> {
    let mut distances: HashMap<usize, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start, 0);
    heap.push(Edge {
        node: start,
        weight: 0,
    });

    while let Some(Edge { node, weight }) = heap.pop() {
        if let Some(&d) = distances.get(&node) {
            if weight > d {
                continue;
            }
        }

        for &(neighbor, edge_weight) in &graph[&node] {
            let new_distance = weight + edge_weight;
            if !distances.contains_key(&neighbor) || new_distance < distances[&neighbor] {
                distances.insert(neighbor, new_distance);
                heap.push(Edge {
                    node: neighbor,
                    weight: new_distance,
                });
            }
        }
    }

    distances
}

fn find_bottlenecks(
    graph: &[(usize, usize, usize)],
    start: usize,
    end: usize,
) -> Vec<(usize, usize, usize)> {
    let mut result = Vec::new();

    // Create graph
    let mut g: Graph = HashMap::new();
    for &(a, b, weight) in graph {
        g.entry(a).or_insert(Vec::new()).push((b, weight));
        g.entry(b).or_insert(Vec::new()).push((a, weight));
    }

    let shortest_path = dijkstra(&g, start);

    for &(a, b, weight) in graph {
        let mut new_graph = g.clone();
        new_graph.entry(a).and_modify(|e| e.retain(|x| x.0 != b));
        new_graph.entry(b).and_modify(|e| e.retain(|x| x.0 != a));

        let new_shortest_path = dijkstra(&new_graph, start);

        if !new_shortest_path.contains_key(&end) || shortest_path[&end] < new_shortest_path[&end] {
            result.push((a, b, weight));
        }
    }

    result.sort();
    result
}

fn main() {
    println!("Enter tuples in the form [(a, b, c), (d, e, f), ...]:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let start_index = input.find('[').expect("Square brackets not found");
    let end_index = input.rfind(']').expect("Square brackets not found");

    let data_str = &input[start_index..=end_index];

    let start = input[end_index + 3..=end_index + 3]
        .parse::<usize>()
        .unwrap();
    let end = input[end_index + 6..=end_index + 6]
        .parse::<usize>()
        .unwrap();

    let mut data: Vec<(usize, usize, usize)> = Vec::new();

    let mut buffer = String::new();
    let mut inside_brackets = false;

    for c in data_str.chars() {
        match c {
            '[' => inside_brackets = true,
            ']' => inside_brackets = false,
            '(' => buffer.clear(),
            ')' => {
                let tuple: Vec<usize> = buffer
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                data.push((tuple[0], tuple[1], tuple[2]));
            }
            _ => {
                if inside_brackets {
                    buffer.push(c);
                }
            }
        }
    }

    println!("{:?}", find_bottlenecks(&data[..], start, end));
}
