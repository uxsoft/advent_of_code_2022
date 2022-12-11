use std::collections::HashMap;

pub struct IdGenerator {
    previous_id: usize
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator {
            previous_id: 0
        }
    }
    
    pub fn new_id(&mut self) -> usize {
        self.previous_id += 1;
        self.previous_id
    }
}

pub struct ElFSNode {
    id: usize,
    name: String,
    files: HashMap<String, u32>,
    parent: Option<usize>,
    directories: Vec<usize>
}

impl ElFSNode {
    pub fn new(id: usize, name: String, parent: Option<usize>) -> ElFSNode {
        ElFSNode {
            id,
            name,
            parent,
            files: HashMap::new(),
            directories: Vec::new(),
        }
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
    let mut idgen = IdGenerator::new();
    let mut nodes = HashMap::new();
    
    let root = ElFSNode::new(idgen.new_id(), "root".to_string(), None);
    nodes.insert(root.id, root);
    
    let mut current_dir = &root.id;

    for cmd in commands {
        let command = ElFSCommand::parse(cmd).expect("Could not parse the command");
        match command {
            ElFSCommand::CdRoot => {
                current_dir = &root.id;
                println!("Changed dir to root");
            }
            ElFSCommand::CdUp => {
                let parent_id = nodes.get(current_dir).unwrap().parent.expect("Can't go up when there is no parent");
                current_dir = &parent_id;
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
                        let new_node = ElFSNode::new(idgen.new_id(), "dir_name".to_string(), Some(*current_dir));
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
