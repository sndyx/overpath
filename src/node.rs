use std::collections::HashMap;

struct Map {
    nodes: HashMap<u32, Node>
}

impl Map {

    fn from_data() {

    }

}

struct Node {
    x: u32,
    y: u32,
    children: Vec<u32>
}

impl Node {

    fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            children: vec![],
        }
    }

    fn add_node(&mut self, node: u32) {
        self.children.push(node)
    }

}