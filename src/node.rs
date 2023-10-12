use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    elements: Vec<Element>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Element {
    #[serde(rename = "node")]
    Node { id: u64, lat: f32, lon: f32 },
    #[serde(rename = "way")]
    Way { nodes: Vec<u64> }
}

impl Element {
    fn is_node(&self) -> bool {
        matches!(self, Element::Node { id: _, lat: _, lon: _ })
    }
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub lat: f32,
    pub lon: f32,
    pub connections: Vec<u64>
}

impl Node {
    fn new() -> Self {
        Self { lat: 0.0, lon: 0.0, connections: vec![] }
    }
}

#[derive(Debug, PartialEq)]
pub struct Map {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
    pub nodes: HashMap<u64, Node>,
}

impl Map {
    pub fn from_json(str: &str) -> Self {
        let mut map: HashMap<u64, Node> = HashMap::new();
        let data: Data = serde_json::from_str(str)
            .expect("Error reading export.json!");
        let mut x1: f32 = 0.0;
        let mut x2: f32 = 0.0;
        let mut y1: f32 = 0.0;
        let mut y2: f32 = 0.0;
        for element in data.elements.iter() {
            match element {
                Element::Node { id, lat, lon } => {
                    let entry = map.entry(*id).or_insert(Node::new());
                    entry.lat = *lat;
                    entry.lon = *lon;
                    if entry.lat < x1 { x1 = entry.lat }
                    if entry.lat > x2 { x2 = entry.lat }
                    if entry.lon < y1 { y1 = entry.lon }
                    if entry.lon > y2 { y2 = entry.lon }
                },
                Element::Way { nodes } => {
                    for (i, node) in nodes.iter().enumerate() {
                        let entry = map.entry(*node).or_insert(Node::new());
                        let before = if i == 0 { nodes.len() - 1 } else { i - 1 };
                        let after = if i + 1 == nodes.len() { 0 } else { i + 1 };
                        entry.connections.push(nodes[before]);
                        entry.connections.push(nodes[after]);
                    }
                }
            }
        }
        Self {
            x1, x2, y1, y2,
            nodes: map
        }
    }
}