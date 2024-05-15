use std::ops::Deref;
use std::{borrow::Borrow, collections::VecDeque};

use rand;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::fmt::{self, Display};
#[derive(Default)]
pub struct Graph {
    pub size: usize,
    pub nodes: VecDeque<Vec<Node>>,
}
/// (vertex, edge, weight)
pub type Node = (usize, usize, u8);

impl Graph {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            nodes: vec![vec![(0xFF, 0xFF, 0xFF); size]; size].into(),
        }
    }
    pub fn add_edge(&mut self, vertex: usize, edge: usize, weight: u8) {
        self.nodes[vertex][edge] = (vertex, edge, weight);
    }
    pub fn print(&self) {
        for (vertex, edges) in self.nodes.iter().enumerate() {
            print!("{} -> ", vertex);
            for &(v, e, w) in edges {
                if v != 0xFF {
                    print!("({},{},{}) ", v, e, w);
                }
            }
            println!();
        }
    }
    pub fn print_g(&self) {
        for (vertex, edges) in self.nodes.iter().enumerate() {
            print!("{} -> ", vertex);
            for &(v, _, _) in edges {
                if v != 0xFF {
                    print!("{} ", v);
                }
            }
            println!();
        }
    }
    pub fn djk_spt(&mut self, src: usize, dst: usize) -> BTreeMap<usize, usize> {
        let mut shortest = BTreeMap::new();
        for i in 0..self.size {
            shortest.insert(i, 0xFF);
        }
        shortest.insert(src, 0);
        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, src)));

        while min_heap.len() > 0 {
            let (w1, v1) = min_heap.pop().unwrap().0;
            // if shortest.contains_key(&v1) {
            //     continue;
            // }
            // if v1 == 0xFF {
            //     continue;
            // }
            if w1 > shortest[&v1] {
                continue;
            }
            // println!("v1: {:?}", v1);
            for (_, v2, w2) in self.nodes[v1]
                .clone()
                .into_iter()
                .filter(|item| item.0 != 0xFF)
                .collect::<Vec<Node>>()
            {
                let new_dist = w1 + (w2 as usize);

                // println!("v1: {:?}, v2: {:?}, w1: {:?}, w2: {:?}", v1, v2, w1, w2);
                if new_dist < shortest[&v2] {
                    shortest.insert(v2, new_dist);
                    min_heap.push(Reverse((new_dist, v2)));
                }
            }
        }
        for key in shortest.keys() {
            println!("{:?} has {:?}", key, shortest[key]);
        }
        shortest
    }
    pub fn djk_simple(&mut self, src: usize, dst: usize) -> Vec<usize> {
        let mut dist: Vec<usize> = vec![0xFF; self.size];
        let mut pred: Vec<usize> = vec![0xFF; self.size];
        dist[src] = 0;
        for u in 0..self.size {
            let edges: Vec<Node> = self.nodes[u]
                .clone()
                .into_iter()
                .min_by(|x, y| x.2.cmp(&y.2))
                .and_then(|val| Some(vec![val]))
                .unwrap_or(vec![])
                .into_iter()
                .filter(|(v, _, _)| *v != 0xFF)
                .collect();
            println!("edge: {:?}", edges);
            for (u, v, w) in edges {
                if dist[v] > dist[u] + (w as usize) {
                    dist[v] = dist[u] + (w as usize);
                    pred[v] = u;
                    // if v == dst {
                    //     break;
                    // }
                }
                // println!("u: {} v:{:?}", u, self.nodes.clone().len());
            }
        }
        println!("Distance: {:?}", dist);
        dist
    }
    pub fn bfs_spt(&mut self, src: usize, dst: usize) -> Vec<usize> {
        let mut dist: Vec<usize> = vec![0xFF; self.size];
        let mut pred: Vec<usize> = vec![0xFF; self.size];
        dist[src] = 0;
        for u in 0..self.size {
            let edges: Vec<Node> = self
                .nodes
                .pop_front()
                .unwrap_or(vec![])
                .into_iter()
                .filter(|(v, _, _)| *v != 0xFF)
                .collect();
            for (u, v, w) in edges {
                if dist[v] > dist[u] + (w as usize) {
                    dist[v] = dist[u] + (w as usize);
                    pred[v] = u;
                    if v == dst {
                        break;
                    }
                }
                // println!("u: {} v:{:?}", u, self.nodes.clone().len());
            }
        }
        println!("Distance: {:?}", dist);
        print!("Path: ");
        for (i, _) in &pred
            .iter()
            .enumerate()
            // .rev()
            .filter(|(_, parent)| **parent != 0xFF)
            .collect::<Vec<_>>()
        {
            print!("{} ", pred.get(i + 1).unwrap_or(&dst));
        }
        dist
    }
    pub fn bfs_traverse(self, start: usize) -> Vec<usize> {
        let mut visited: Vec<usize> = vec![];
        visited.push(start);
        for (_, edges) in self.nodes.iter().enumerate() {
            let edges: Vec<&Node> = edges.into_iter().filter(|(v, _, _)| *v != 0xFF).collect();
            for e in edges {
                let is_visited = visited.iter().find(|n| **n == e.1).is_some();
                if !is_visited && e.0 >= start {
                    visited.push(e.1);
                }
            }
        }
        println!("Traverse: {:?}", visited);
        visited
    }
}

#[derive(Clone)]
pub struct BinaryTreeNode<T: Ord> {
    pub left: Option<Box<BinaryTreeNode<T>>>,
    pub right: Option<Box<BinaryTreeNode<T>>>,
    pub data: Option<T>,
}
pub enum Traversal {
    InOrder,
    PreOrder,
    PostOrder,
}

impl<T: Ord + Clone + Display + fmt::Debug> BinaryTreeNode<T> {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            data: None,
        }
    }
    pub fn insert(&mut self, data: T) {
        match &self.data {
            None => {
                self.data = Some(data);
            }
            Some(data_inner) => {
                let pointer = if *data_inner > data {
                    &mut self.left
                } else {
                    &mut self.right
                };
                match pointer {
                    Some(ref mut child) => {
                        child.insert(data);
                    }
                    None => {
                        *pointer = Some(Box::new(BinaryTreeNode {
                            left: None,
                            right: None,
                            data: Some(data),
                        }));
                    }
                }
            }
        }
    }
    pub fn traverse(&self, config: Traversal) {
        match config {
            Traversal::InOrder => match &self.data {
                None => println!("empty"),
                Some(root_data) => {
                    if let Some(left) = &self.left {
                        left.traverse(Traversal::InOrder);
                    }
                    print!("{},", root_data);
                    if let Some(right) = &self.right {
                        right.traverse(Traversal::InOrder);
                    }
                }
            },
            Traversal::PreOrder => match &self.data {
                None => println!("empty"),
                Some(root_data) => {
                    print!("{},", root_data);
                    if let Some(left) = &self.left {
                        left.traverse(Traversal::PreOrder);
                    }
                    if let Some(right) = &self.right {
                        right.traverse(Traversal::PreOrder);
                    }
                }
            },

            _ => unimplemented!("not implemented"),
        }
    }
    pub fn search(&self, target: T) {
        match &self.data {
            None => println!("Empty Tree"),
            Some(root_node) => {
                if *root_node > target {
                    if let Some(inner) = &self.left {
                        inner.search(target);
                    } else {
                        println!("{} not found in tree", target);
                    }
                } else if *root_node < target {
                    if let Some(inner) = &self.right {
                        inner.search(target);
                    } else {
                        println!("{} not found in tree", target);
                    }
                } else if *root_node == target {
                    println!("found {} at {:?}", root_node, &self);
                } else {
                    println!("{} not found in tree", target);
                }
            }
        }
    }
}
impl<T> fmt::Debug for BinaryTreeNode<T>
where
    T: Ord + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_subtree<T>(
            tree: &Option<Box<BinaryTreeNode<T>>>,
            f: &mut fmt::Formatter<'_>,
            indent: usize,
            branch: &str,
        ) -> fmt::Result
        where
            T: Ord + fmt::Debug,
        {
            if let Some(node) = tree {
                writeln!(f, "{}{}: {:?}", "  ".repeat(indent), branch, node.data)?;
                fmt_subtree(&node.left, f, indent + 1, "left")?;
                fmt_subtree(&node.right, f, indent + 1, "right")?;
            } else {
                writeln!(f, "{}{}: None", "  ".repeat(indent), branch)?;
            }
            Ok(())
        }

        writeln!(f, "BinarySearchTree:")?;
        if self.data.is_some() {
            writeln!(f, "root: {:?}", self.data)?;
            fmt_subtree(&self.left, f, 1, "left")?;
            fmt_subtree(&self.right, f, 1, "right")?;
        } else {
            writeln!(f, "root: None")?;
        }
        Ok(())
    }
}
// const SIZE: usize = 9;

fn main() {
    let mut g = Graph::new(4);
    g.add_edge(0, 1, 2);
    g.add_edge(0, 2, 2);
    g.add_edge(1, 2, 1);
    g.add_edge(1, 3, 1);
    g.add_edge(2, 0, 1);
    g.add_edge(2, 3, 2);
    g.add_edge(3, 3, 2); // g.add_edge(2, 3, 2);
    g.add_edge(3, 3, 2);
    g.print();
    g.bfs_traverse(3);

    let mut graph = Graph::new(9);
    graph.add_edge(0, 1, 4);
    graph.add_edge(0, 7, 8);
    graph.add_edge(1, 2, 8);
    graph.add_edge(1, 7, 11);
    graph.add_edge(2, 3, 7);
    graph.add_edge(2, 8, 2);
    graph.add_edge(2, 5, 4);
    graph.add_edge(3, 4, 9);
    graph.add_edge(3, 5, 14);
    graph.add_edge(4, 5, 10);
    graph.add_edge(5, 6, 2);
    graph.add_edge(6, 7, 1);
    graph.add_edge(6, 8, 6);
    graph.add_edge(7, 8, 7);
    graph.print();
    graph.djk_spt(0, 3);

    let mut bst = BinaryTreeNode::new();
    bst.insert(100);
    bst.insert(20);
    bst.insert(200);
    bst.insert(10);
    bst.insert(30);
    bst.insert(150);
    bst.insert(300);
    // println!("bst: {:?}", bst);
    bst.traverse(Traversal::InOrder);
    print!("\n");
    bst.traverse(Traversal::PreOrder);
    bst.search(89);
}
