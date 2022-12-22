use std::{collections::HashMap, rc::Rc, cell::RefCell};

#[derive(Debug)]
pub enum Operation {
    Ls(String),
    CdUp,
    CdRoot,
    Cd(String),
}

pub struct Node {
    name: String,
    size: RefCell<u32>,
    parent: Option<Rc<Node>>,
    files: RefCell<HashMap<String, u32>>,
    dirs: RefCell<HashMap<String, Rc<Node>>>
}

impl Node {
    pub fn new(name: String, parent: Option<Rc<Node>>) -> Self {
        Node {
            name, 
            size: RefCell::new(0), 
            parent,
            files: RefCell::new(HashMap::new()),
            dirs: RefCell::new(HashMap::new())
        }
    }

    pub fn pretty_print(&self) {
        print!("Node(name={}, dirs=[ ", self.name);
        for i in self.dirs.borrow().keys() {
            print!("{} ", i);
        }
        println!("])");
    }
    
    pub fn size(&self) -> u32 {
        let files_size: u32 = self.files.borrow().values().sum();
        let dirs_size: u32 = self.dirs.borrow_mut().values().map(|dir| dir.size()).sum();
        
        let size = files_size + dirs_size;
        self.size.replace(size);
        size
    }
    
    pub fn descendants(root: Rc<Node>) -> Vec<Rc<Node>> {
        let mut to_visit = vec![root.clone()];
        let mut items = vec![root.clone()];
        while let Some(dir) = to_visit.pop() {
            for d in dir.dirs.borrow().values() {
                to_visit.push(d.clone());
            }
            items.push(dir.clone());
        }
        items
    }
}

impl Operation {
    pub fn parse(input: &str) -> Result<Operation, String> {
        let input = input.trim();
        if input.starts_with("ls") {
            let dir = input.trim_start_matches("ls\r\n").to_string();
            Ok(Operation::Ls(dir))
        } else if input.starts_with("cd /") {
            Ok(Operation::CdRoot)
        } else if input.starts_with("cd ..") {
            Ok(Operation::CdUp)
        } else if input.starts_with("cd ") {
            let dir = input.trim_start_matches("cd ").to_string();
            Ok(Operation::Cd(dir))
        } else {
            Err(input.to_string())
        }
    }
}

pub fn process(input: String) {
    let commands: Vec<&str> = input.split("$ ").skip(1).collect();
    
    let root = Rc::new(Node::new("/".to_string(), None));
    let mut cwd = root.clone();
    
    for cmd in commands {
        let op = Operation::parse(cmd).expect("Could not parse the command");
        match op {
            Operation::CdRoot => {
                println!("Changing dir to root");
                cwd = root.clone();
            },
            Operation::CdUp => {
                println!("Changing dir to parent");
                cwd = cwd.parent.as_ref().unwrap().clone();
            },
            Operation::Cd(target_dir_name) => {
                println!("Changing dir to named child ({})", target_dir_name);

                let new_dir = cwd.dirs.borrow().get(&target_dir_name).unwrap().clone();
                cwd = new_dir;
            },
            Operation::Ls(ls) => {
                for line in ls.lines() {
                    let words: Vec<&str> = line.split(" ").collect();
                    match (words[0], words[1]) {
                        ("dir", dir_name) => {
                            println!("Creating a dir {} in dir {}", dir_name, cwd.name);

                            cwd.dirs.borrow_mut().insert(
                                dir_name.to_string(), 
                                Rc::new(Node::new(dir_name.to_string(), Some(cwd.clone())))
                            );
                        },
                        (size, file_name) => {    
                            println!("Creating a file {} in dir {}", file_name, cwd.name);
                            
                            let size = size.parse::<u32>().expect("Size of a file isn't a number");
                            cwd.files.borrow_mut().insert(file_name.to_string(), size);
                        }
                    };
                }
            }
        };
    }
    
    let used_size = root.size();
    let update_size: u32 = 30000000;
    let system_size: u32 = 70000000;
    let space_needed: u32 = used_size + update_size - system_size;

    println!("Space used: {}", used_size);
    println!("Space needed: {}", space_needed);

    let total_under_threshold: u32 = Node::descendants(root.clone())
        .iter()
        .map(|n| *n.size.borrow())
        .filter(|v| *v < 100000)
        .sum();

    println!("Total of dirs under 100000: {}", total_under_threshold);

    let dir_size_to_delete: u32 = Node::descendants(root.clone())
        .iter()
        .map(|n| *n.size.borrow())
        .filter(|v| *v > space_needed)
        .min()
        .unwrap();
    println!("Dir to delete: {}", dir_size_to_delete);
}

#[cfg(test)]
mod tests_part7 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_n4() {}
}
