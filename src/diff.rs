use dom::{Id, Node};
use patch::Patch;

pub fn diff(src: Node, tar: Node) -> Vec<Patch> {
    let id = Vec::new();
    return diff_with_id(id, src, tar);
}

fn diff_with_id(id: Id, src: Node, tar: Node) -> Vec<Patch> {
    let mut patches: Vec<Patch> = Vec::new();

    if src.tag != tar.tag {
        patches.push(Patch::ReplaceNode(id, tar))
    } else {

        // Loop over old attributes and check for deletions and replacements.
        for (key, src_val) in src.attributes.iter() {
            match tar.attributes.get(key) {
                None => patches.push(Patch::RemoveAttribute(id.clone(), key.to_string())),
                Some(tar_val) => {
                    if src_val != tar_val {
                        patches.push(Patch::SetAttribute(id.clone(), key.to_string(), tar_val.clone()))
                    }
                }
            }
        }

        // Loop over new attributes and check for additions.
        for (key, val) in tar.attributes.iter() {
            if !src.attributes.contains_key(key) {
                patches.push(Patch::SetAttribute(id.clone(), key.to_string(), val.clone()))
            }
        }
    
        // Add new children if necessary.
        if src.children.len() < tar.children.len() {
            for n in 0..(tar.children.len() - src.children.len()) {
                let index = n + src.children.len();
                let new_id = [id.clone(), vec![index]].concat();
                patches.push(Patch::AppendNode(new_id, tar.children[index].clone()));
            }
        }

        // Loop the rest of the children.
        for (index, src_child) in src.children.iter().enumerate() {
            let new_id = [id.clone(), vec![index]].concat();
            match tar.children.get(index) {
                None => patches.push(Patch::RemoveNode(new_id)),
                Some(tar_child) => patches.append(&mut diff_with_id(new_id, src_child.clone(), tar_child.clone()))
            }
        }
    }

    return patches;
}

#[cfg(test)]
mod tests {
    use dom::Attribute;
    use super::*;

    #[test]
    fn diff_same() {
        let src = Node::new("test".to_string());
        let tar = src.clone();
        let patches = diff(src, tar);

        assert_eq!(patches.len(), 0);
    }

    #[test]
    fn diff_simple_tag() {
        let src = Node::new("test".to_string());
        let tar = Node::new("text".to_string());
        let patches = diff(src, tar.clone());

        assert_eq!(patches.len(), 1);
        assert_eq!(patches[0], Patch::ReplaceNode(vec![], tar.clone()));
    }

    #[test]
    fn diff_simple_attributes() {
        let mut src = Node::new("test".to_string());
        src.attributes.insert("type".to_string(), Attribute::String("test".to_string()));
        src.attributes.insert("only_src".to_string(), Attribute::Bool(true));

        let mut tar = Node::new("test".to_string());
        tar.attributes.insert("type".to_string(), Attribute::String("text".to_string()));
        tar.attributes.insert("only_tar".to_string(), Attribute::Bool(true));

        let patches = diff(src, tar);

        assert_eq!(patches.len(), 3);
        assert!(patches.contains(&Patch::SetAttribute(vec![], "type".to_string(), Attribute::String("text".to_string()))));
        assert!(patches.contains(&Patch::RemoveAttribute(vec![], "only_src".to_string())));
        assert!(patches.contains(&Patch::SetAttribute(vec![], "only_tar".to_string(), Attribute::Bool(true))));
    }

    #[test]
    fn diff_nested_tag() {
        let mut src = Node::new("test0".to_string());
        src.children.push(Node::new("test1".to_string()));
        src.children.push(Node::new("test2".to_string()));
        src.children.push(Node::new("test3".to_string()));

        let mut tar = Node::new("test0".to_string());
        tar.children.push(Node::new("test1".to_string()));
        tar.children.push(Node::new("test3".to_string()));

        let patches = diff(src.clone(), tar.clone());

        assert_eq!(patches.len(), 2);
        assert!(patches.contains(&Patch::RemoveNode(vec![2])));
        assert!(patches.contains(&Patch::ReplaceNode(vec![1], tar.children[1].clone())));
    }

    #[test]
    fn diff_nested_attribute() {
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

        let patches = diff(src.clone(), tar.clone());

        assert_eq!(patches.len(), 3);
        assert!(patches.contains(&Patch::SetAttribute(vec![2], "type".to_string(), Attribute::String("text".to_string()))));
        assert!(patches.contains(&Patch::RemoveAttribute(vec![2], "only_src".to_string())));
        assert!(patches.contains(&Patch::SetAttribute(vec![2], "only_tar".to_string(), Attribute::Bool(true))));
    }
}
