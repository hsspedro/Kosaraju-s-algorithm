# Kosaraju's Algorithm in Rust

This repository contains an implementation of Kosaraju's algorithm in Rust. Kosaraju's algorithm is used to find the strongly connected components (SCCs) of a directed graph.

## Overview

Kosaraju's algorithm works in two main phases:
1. Perform a Depth-First Search (DFS) on the original graph to determine the finish order of nodes.
2. Transpose the graph (reverse the direction of all edges) and perform DFS in the order of decreasing finish times to identify SCCs.

## Features

- Efficiently finds all strongly connected components in a directed graph.
- Uses Rust's standard library collections for graph representation.
- Demonstrates the use of DFS and graph transposition.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system. You can download them from rust-lang.org.

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/kosaraju-rust.git
    cd kosaraju-rust
    ```

2. Build the project:
    ```sh
    cargo build
    ```

3. Run the project:
    ```sh
    cargo run
    ```

### Usage

You can modify the `main` function in `src/main.rs` to test the algorithm with different graphs. Here is an example of how to define a graph and run the algorithm:

```rust
fn main() {
    let mut graph = HashMap::new();
    graph.insert(1, vec![2]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![1]);
    graph.insert(3, vec![4]);
    graph.insert(4, vec![5]);
    graph.insert(5, vec![4]);

    let scc = kosaraju(&graph);
    println!("Strongly Connected Components: {:?}", scc);
}
