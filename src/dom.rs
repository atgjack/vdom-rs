use std::collections::HashMap;
use std::string::ToString;
use html5ever::rcdom::{RcDom, NodeData, Handle};
use html5ever::{parse_fragment, tree_builder, QualName};
use html5ever::tendril::TendrilSink;

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

    fn from_dom(node: &Handle) -> Option<Node> {
        if let NodeData::Element { ref name, ref attrs, .. } = node.data {
            let tag = name.local.to_string();
            if tag != "html" {
                return Some(Node {
                    tag: name.local.to_string(),
                    children: node.children.borrow().iter().filter_map(Node::from_dom).collect(),
                    attributes: attrs.borrow().iter().map(attr_to_hashmap).collect(),
                });
            }          
        }

        if node.children.borrow().iter().len() > 0 {
            Node::from_dom(node.children.borrow().first().unwrap())
        } else {
            None
        }
    }

    pub fn from_html(html: String) -> Option<Node> {
        let qn = QualName::new(Default::default(), Default::default(), Default::default());
        let dom = parse_fragment(RcDom::default(), Default::default(), qn, Vec::new())
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .unwrap();
        Node::from_dom(&dom.document)
    }
}

fn attr_to_hashmap(attr: &tree_builder::Attribute) -> (String, Attribute) {
    (attr.name.local.to_string(), Attribute::String(attr.value.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node::new("test".to_string());

        assert_eq!(node.tag, "test".to_string());
    }

    #[test]
    fn read_html() {
        let html = "<div class='test'>".to_string();
        let mut node = Node::new("div".to_string());
        node.attributes.insert("class".to_string(), Attribute::String("test".to_string()));

        assert_eq!(Node::from_html(html).unwrap(), node);
    }

    #[test]
    fn read_html_nested() {
        let mut node = Node::new("div".to_string());
        node.children.push(Node::new("div".to_string()));
        node.children.push(Node::new("div".to_string()));
        node.children.push(Node::new("div".to_string()));
        node.children[2].attributes.insert("class".to_string(), Attribute::String("test".to_string()));

        let html = "<div><div /><div /><div class='test'/>".to_string();
        assert_eq!(Node::from_html(html).unwrap(), node);
    }
}
