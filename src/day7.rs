use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

pub struct Node {
    name: String,
    files: HashMap<String, u32>,
    parent: Option<Weak<RefCell<Node>>>,
    directories: Vec<Rc<RefCell<Node>>>,
}

type NodePtr = Rc<RefCell<Node>>;

pub struct Tree {
    root: NodePtr
}

impl Node {
    pub fn new(name: String, parent: Option<NodePtr>) -> Node {
        Node {
            name,
            parent: parent.map(|x| Rc::downgrade(&x)),
            files: HashMap::new(),
            directories: Vec::new(),
        }
    }
}

impl From<Node> for NodePtr {
    fn from(node: Node) -> NodePtr {
        Rc::new(RefCell::new(node))
    }
}

#[derive(Debug)]
pub enum ElFSCommand {
    Ls(String),
    CdUp,
    CdRoot,
    Cd(String),
}

impl ElFSCommand {
    pub fn parse(input: &str) -> Result<ElFSCommand, String> {
        let input = input.trim();
        if input.starts_with("ls") {
            let dir = input.trim_start_matches("ls\r\n").to_string();
            Ok(ElFSCommand::Ls(dir))
        } else if input.starts_with("cd /") {
            Ok(ElFSCommand::CdRoot)
        } else if input.starts_with("cd ..") {
            Ok(ElFSCommand::CdUp)
        } else if input.starts_with("cd ") {
            let dir = input.trim_start_matches("cd ").to_string();
            Ok(ElFSCommand::Cd(dir))
        } else {
            Err(input.to_string())
        }
    }
}

pub fn process(input: String) {
    let commands: Vec<&str> = input.split("$").skip(1).collect();

    let root = Node::new("root".to_string(), None);
    let mut current_dir : NodePtr = root.into();

    for cmd in commands {
        let command = ElFSCommand::parse(cmd).expect("Could not parse the command");
        match command {
            ElFSCommand::CdRoot => {
                current_dir = root.into();
                println!("Changed dir to root");
            }
            ElFSCommand::CdUp => {
                let parent = current_dir.borrow().parent.expect("This directory has no parent").upgrade().unwrap();
                current_dir = parent;
            }
            ElFSCommand::Cd(target_dir_name) => {
                let child = nodes.get(current_dir).unwrap().directories.iter()
                    .find(|x| { nodes.get(x).unwrap().name == target_dir_name })
                    .expect("Tried to change into a sub-directory which doesn't exist in the current directory");
                current_dir = child;
            }
            ElFSCommand::Ls(lsoutput) => {
                for line in lsoutput.lines() {
                    if line.starts_with("dir") {
                        let dir_name = line.trim_start_matches("dir ");
                        let new_node = Node::new(
                            idgen.new_id(),
                            "dir_name".to_string(),
                            Some(*current_dir),
                        );
                        nodes.insert(new_node.id, new_node);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_part7 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_n4() {}
}
