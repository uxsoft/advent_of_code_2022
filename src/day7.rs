use std::collections::HashMap;

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

type NodeId = usize;

pub fn id(counter: &mut NodeId) -> NodeId {
    *counter += 1;
    *counter
}

pub fn calc_sizes(node_id: NodeId, node_files: &HashMap<NodeId, Vec<(String, u32)>>, node_dirs: &HashMap<NodeId, Vec<NodeId>>, node_size: &mut HashMap<NodeId, u32>) -> u32 {
    let dir_size: u32 = node_dirs
        .get(&node_id)
        .unwrap().iter()
        .map(|i| calc_sizes(*i, node_files, node_dirs, node_size))
        .sum();
    let file_size: u32 = node_files
        .get(&node_id)
        .unwrap().iter()
        .map(|i| i.1)
        .sum();
    
    let total_size = file_size + dir_size;
    node_size.insert(node_id, total_size);
    total_size
}

pub fn process(input: String) {
    let commands: Vec<&str> = input.split("$").skip(1).collect();
    
    let mut next_id: NodeId = 0;
    let mut node_names: HashMap<NodeId, String> = HashMap::new();
    let mut node_parent: HashMap<NodeId, NodeId> = HashMap::new();
    let mut node_files: HashMap<NodeId, Vec<(String, u32)>> = HashMap::new();
    let mut node_dirs: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    let mut node_size: HashMap<NodeId, u32> = HashMap::new();
    
    let root_id = id(&mut next_id);
    node_names.insert(root_id, "root".to_string());
    node_files.insert(root_id, Vec::new());
    node_dirs.insert(root_id, Vec::new());
    
    let mut current_dir_id = root_id;

    for cmd in commands {
        let command = ElFSCommand::parse(cmd).expect("Could not parse the command");
        match command {
            ElFSCommand::CdRoot => {
                println!("Changing dir to root");
                current_dir_id = root_id;
            }
            ElFSCommand::CdUp => {
                println!("Changing dir to parent");
                let parent_id = node_parent.get(&current_dir_id).expect("This directory has no parent directory, can't cd ..");
                current_dir_id = *parent_id;
            }
            ElFSCommand::Cd(target_dir_name) => {
                println!("Changing dir to named child ({})", target_dir_name);
                let children = node_dirs.get(&current_dir_id).unwrap();
                let child_id = children.iter()
                    .find(|x| { *node_names.get(x).unwrap() == target_dir_name })
                    .expect("Tried to change into a sub-directory which doesn't exist in the current directory");

                current_dir_id = *child_id;
            }
            ElFSCommand::Ls(ls) => {
                for line in ls.lines() {
                    if line.starts_with("dir") { // Directory [dir a]
                        let name = line.trim_start_matches("dir ").to_string();
                        let id = id(&mut next_id);
                        println!("Creating dir {} (id={}) in dir {}", name, id, current_dir_id);
                        
                        // new node
                        node_names.insert(id, name);
                        node_files.insert(id, Vec::new());
                        node_dirs.insert(id, Vec::new());
                        
                        // parent child relationship
                        node_parent.insert(id, current_dir_id);
                        node_dirs.get_mut(&current_dir_id).unwrap().push(id);
                    } else { // File [123 file.txt]
                        let parts : Vec<&str> = line.split(" ").collect();
                        let size = parts[0].parse::<u32>().expect("Size of a file isn't a number");
                        let name = parts[1].to_string();
                        println!("Creating file {} in dir {}", name, current_dir_id);
                        
                        let files = node_files.get_mut(&current_dir_id).unwrap();
                        files.push((name, size));
                    }
                }
            }
        }
    }
    
    let used_size = calc_sizes(root_id, &node_files, &node_dirs, &mut node_size);
    let update_size: u32 = 30000000;
    let system_size: u32 = 70000000;
    let space_needed: u32 = used_size + update_size - system_size;
    
    println!("Space needed: {}", space_needed);
    let dir_size_to_delete = node_size.values().filter(|v| **v > space_needed).min().unwrap();
    println!("Dir to delete: {}", dir_size_to_delete);
}

#[cfg(test)]
mod tests_part7 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_n4() {}
}
