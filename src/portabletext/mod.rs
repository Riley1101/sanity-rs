pub mod blocks;
pub mod renderer;
pub mod tests;

use blocks::Node;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct PortableText {
    pub nodes: Vec<Node>,
}

impl PortableText {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }
}

impl Display for PortableText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.nodes.len())
    }
}
