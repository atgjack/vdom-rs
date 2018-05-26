use std::collections::HashMap;

pub type Id = Vec<usize>;

#[derive(PartialEq, Clone, Debug)]
pub enum Attribute {
    String(String),
    Number(f64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub tag:       String,
    pub children:   Vec<Node>,
    pub attributes: HashMap<String, Attribute>,
}

impl Node {
    pub fn new(tag: String) -> Node {
        Node {
            tag,
            children: Vec::new(),
            attributes: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node::new("test".to_string());

        assert_eq!(node.tag, "test".to_string());
    }
}
