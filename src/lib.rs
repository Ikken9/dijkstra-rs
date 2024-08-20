use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct VertexId(pub char);

impl Ord for VertexId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for VertexId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for VertexId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub id: VertexId,
    pub edges: Vec<Edge>
}

impl PartialOrd<Self> for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.id.cmp(&self.id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    pub to: VertexId,
    pub weight: u32
}

pub struct Graph {
    pub vertices: HashMap<VertexId, Vertex>
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new()
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        let copy = vertex.clone();
        let id = copy.id;
        self.vertices.insert(id, copy);
    }

    pub fn dijkstra_heap(&mut self, start: Vertex) {
        let mut distances: HashMap<VertexId, u32> = HashMap::new();
        let mut visited: HashSet<VertexId> = HashSet::new();

        let mut priority_queue = BinaryHeap::new();

        distances.insert(start.id.clone(), 0);
        priority_queue.push(State { vertex: start.id, cost: 0 });

        while let Some(State { vertex: current_vertex, cost: current_distance }) = priority_queue.pop() {
            if !visited.insert(current_vertex) {
                continue;
            }

            if let Some(v) = self.vertices.get(&current_vertex) {
                for neighbor in &v.edges {
                    if let Some(next) = self.vertices.get(&neighbor.to) {
                        let distance = current_distance + neighbor.weight;

                        if distance < *distances.get(&neighbor.to).unwrap_or(&u32::MAX) {
                            distances.insert(neighbor.to.clone(), distance);
                            priority_queue.push(State { vertex: next.id, cost: distance });
                        }
                    }
                }
            }
        }
    }

    pub fn dijkstra_no_heap(&mut self, start: Vertex) {
        let mut distances: HashMap<VertexId, u32> = HashMap::new();
        let mut visited: HashSet<VertexId> = HashSet::new();

        distances.insert(start.id.clone(), 0);

        let mut current_vertex = start.id.clone();
        let graph_len = self.vertices.keys().len();

        while visited.len() < graph_len {
            visited.insert(current_vertex);
            let current_distance = *distances.get(&current_vertex).unwrap_or(&u32::MAX);

            if let Some(v) = self.vertices.get(&current_vertex) {
                for neighbor in &v.edges {
                    let distance = current_distance + neighbor.weight;

                    if distance < *distances.get(&neighbor.to).unwrap_or(&u32::MAX) {
                        distances.insert(neighbor.to, distance);
                    }
                }
            }

            let next_vertex = self.vertices
                .iter()
                .filter(|(_, v)| !visited.contains(&v.id))
                .min_by_key(|(_, v)| distances.get(&v.id).unwrap_or(&u32::MAX))
                .map(|(_, v)| v.clone());

            match next_vertex {
                Some(v) => current_vertex = v.id,
                None => break,
            }
        }
    }
}

#[derive(Eq, PartialEq)]
struct State {
    vertex: VertexId,
    cost: u32
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
