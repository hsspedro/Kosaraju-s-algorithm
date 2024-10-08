use std::collections::HashMap;
use std::collections::HashSet;

fn kosaraju(graph: &HashMap<i32, Vec<i32>>, start_node: i32) -> Vec<Vec<i32>> {
    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    if !visited.contains(&start_node) {
        dfs1(start_node, graph, &mut visited, &mut stack);
    }

    for &node in graph.keys() {
        if !visited.contains(&node) {
            dfs1(node, graph, &mut visited, &mut stack);
        }
    }
    
    let transposed_graph = transpose(graph);

    visited.clear();
    let mut scc = Vec::new();
    while let Some(node) = stack.pop() {
        if !visited.contains(&node) {
            let mut component = Vec::new();
            dfs2(node, &transposed_graph, &mut visited, &mut component);
            scc.push(component);
        }
    }

    scc
}

fn dfs1(node: i32, graph: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>, stack: &mut Vec<i32>) {
    visited.insert(node);
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                dfs1(neighbor, graph, visited, stack);
            }
        }
    }
    stack.push(node);
}

fn dfs2(node: i32, graph: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>, component: &mut Vec<i32>) {
    visited.insert(node);
    component.push(node);
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                dfs2(neighbor, graph, visited, component);
            }
        }
    }
}

fn transpose(graph: &HashMap<i32, Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut transposed = HashMap::new();
    for (&node, neighbors) in graph {
        for &neighbor in neighbors {
            transposed.entry(neighbor).or_insert_with(Vec::new).push(node);
        }
    }
    transposed
}

fn main() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2, 4, 5]);
    graph.insert(2, vec![3, 6]);
    graph.insert(3, vec![2, 7]);
    graph.insert(4, vec![0, 5]);
    graph.insert(5, vec![6]);
    graph.insert(6, vec![5, 7]);
    graph.insert(7, vec![7]);

    let start_node = 2;
    let scc = kosaraju(&graph, start_node);
    println!("{:?}", scc);
}
