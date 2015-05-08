use std::iter::repeat;
use std::collections::VecDeque;

fn max(a: u32, b: u32) -> u32 { if a > b { a } else { b } }

#[derive(Clone)]
struct Edge {
    vertex: u32,
    weight: f32,
}

impl Edge {
    pub fn new(vertex_: u32, weight_: f32) -> Edge {
        Edge{vertex: vertex_, weight: weight_,}
    }
}

struct Graph {
    nodes: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Graph { Graph{nodes: Vec::new(),} }

    pub fn add_edge(&mut self, src: u32, dst: u32, weight: f32) {
        // only works with rust nightly:
        // if (max(src,dst)) > self.nodes.len() as u32 {
        //  self.nodes.resize((max(src, dst) + 1) as usize, Vec::new());
        //}

        let len = self.nodes.len();
        if (max(src, dst)) > len as u32 {
            let new_len = (max(src, dst) + 1) as usize;
            self.nodes.extend(repeat(Vec::new()).take(new_len - len))
        }
        self.nodes[src as usize].push(Edge::new(dst, weight));
    }
    pub fn bfs(&self, src: u32) {
        let mut current: u32 = src;
        let mut queue: VecDeque<u32> = VecDeque::new();
        let mut visited: Vec<bool> = Vec::new();
        
        // only works with rust nightly:
        // visited.resize(self.nodes.len(), false);
        
        let len = self.nodes.len() as usize;
        visited.extend(repeat(false).take(len));
        queue.push_front(current);
        
        while queue.len() > 0 {
            current = queue.pop_back();
            if !visited[current as usize] {
                println!("current: {}" , current);
                visited[current as usize] = true;
            }

            for n in &self.nodes[current as usize] {
                let neighbor: u32 = n.vertex;
                if !visited[neighbor as usize] { queue.push_front(neighbor); }
            }
        }
    }
    pub fn dfs(&self, src: u32) {
        let mut current: u32 = src;
        let mut stack: Vec<u32> = Vec::new();
        let mut visited: Vec<bool> = Vec::new();
        
        // only works with rust nightly:
        // visited.resize(self.nodes.len(), false);
        
        let len = self.nodes.len() as usize;
        visited.extend(repeat(false).take(len));
        stack.push(current);
        
        while stack.len() > 0 {
            current = stack[0 as usize];
            stack.remove(0);
            if !visited[current as usize] {
                println!("current: {}" , current);
                visited[current as usize] = true;
            }

            for n in &self.nodes[current as usize] {
                let neighbor: u32 = n.vertex;
                if !visited[neighbor as usize] { stack.push(neighbor); }
            }
        }
    }
}

fn main() {
    let mut g1 = Graph::new();
    g1.add_edge(0, 1, 1.0);
    g1.add_edge(0, 2, 4.0);
    g1.add_edge(1, 0, 1.0);
    g1.add_edge(1, 2, 2.0);
    g1.add_edge(1, 3, 6.0);
    g1.add_edge(2, 0, 4.0);
    g1.add_edge(2, 1, 2.0);
    g1.add_edge(2, 3, 3.0);
    g1.add_edge(3, 1, 6.0);
    g1.add_edge(3, 2, 3.0);
    g1.bfs(0);
}