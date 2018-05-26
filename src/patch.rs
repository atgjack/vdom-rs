use dom::{Id, Node, Attribute};

#[derive(Debug, PartialEq)]
pub enum Patch {
    RemoveNode(Id),
    AppendNode(Id, Node),
    ReplaceNode(Id, Node),
    RemoveAttribute(Id, String),
    SetAttribute(Id, String, Attribute),
}

pub fn apply(patches: Vec<Patch>, src: Node) -> Result<Node, &'static str> {
    let mut tar = src.clone();
    for patch in patches {
        match patch {
            Patch::RemoveNode(id) => {
                if id.len() == 0 {
                    return Err("Cannot remove root node.");
                }
                let parent_id = id[0..id.len()].to_vec();
                let node_id = *id.last().unwrap();
                if let Some(mut parent) = find_node(parent_id, &mut tar) {
                    if node_id < parent.children.len()  {
                        parent.children.remove(node_id);
                    } else {
                        return Err("Cannot remove child that does not exist.");
                    }
                } else {
                    return Err("Parent node does not exist.");
                }
            },
            Patch::AppendNode(id, node) => {
                if let Some(mut parent) = find_node(id, &mut tar) {
                    parent.children.push(node);
                } else {
                    return Err("Parent node does not exist.");
                }
            },
            Patch::ReplaceNode(id, node) => {
                if id.len() == 0 {
                    tar = node;
                } else {
                    let parent_id = id[0..id.len()].to_vec();
                    let node_id = *id.last().unwrap();
                    if let Some(mut parent) = find_node(parent_id, &mut tar) {
                        parent.children[node_id] = node;
                    } else {
                        return Err("Parent node does not exist.");
                    }
                }
            },
            Patch::RemoveAttribute(id, key) => {
                if let Some(mut node) = find_node(id, &mut tar) {
                    node.attributes.remove(&key);
                } else {
                    return Err("Node does not exist.");
                }
            },
            Patch::SetAttribute(id, key, value) => {
                if let Some(mut node) = find_node(id, &mut tar) {
                    node.attributes.insert(key, value);
                } else {
                    return Err("Node does not exist.");
                }
            },
        }
    }

    return Ok(tar);
}

fn find_node(id: Id, node: &mut Node) -> Option<&mut Node> {
    id.iter().fold(Some(node), |curr, &index| {
        match curr {
            Some(n) => n.children.get_mut(index),
            None => None
        }
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use diff::*;

    fn test_apply(src: Node, tar: Node) {
        let patches = diff(src.clone(), tar.clone());
        let update = apply(patches, src.clone()).unwrap();
        assert_eq!(tar.clone(), update);
    }

    #[test]
    fn patch_empty() {
        let src = Node::new("test".to_string());
        let tar = src.clone();
        test_apply(src, tar);
    }
    
    #[test]
    fn patch_simple() {
        let src = Node::new("test".to_string());
        let tar = Node::new("text".to_string());
        test_apply(src, tar);
    }

    #[test]
    fn patch_nested() {
        let mut src = Node::new("test".to_string());
        src.children.push(Node::new("test".to_string()));
        src.children.push(Node::new("test".to_string()));
        src.children.push(Node::new("test".to_string()));
        src.children[2].attributes.insert("type".to_string(), Attribute::String("test".to_string()));
        src.children[2].attributes.insert("only_src".to_string(), Attribute::Bool(true));

        let mut tar = Node::new("test".to_string());
        tar.children.push(Node::new("test".to_string()));
        tar.children.push(Node::new("test".to_string()));
        tar.children.push(Node::new("test".to_string()));
        tar.children[2].attributes.insert("type".to_string(), Attribute::String("text".to_string()));
        tar.children[2].attributes.insert("only_tar".to_string(), Attribute::Bool(true));
        
        test_apply(src, tar);
    }
}