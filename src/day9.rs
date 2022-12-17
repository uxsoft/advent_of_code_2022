#[derive(Clone, Debug)]
pub enum Operation {
    Right,
    Up,
    Left,
    Down
}

impl Operation {
    pub fn parse(input: &str) -> Result<Operation, String> {
        match input {
            "R" => Ok(Operation::Right),
            "U" => Ok(Operation::Up),
            "L" => Ok(Operation::Left),
            "D" => Ok(Operation::Down),
            i => Err(format!("Unable to parse Operation from {}", i))
        }
    }
}

pub fn process(input: String) {
    let operations: Vec<_>= input
        .lines()
        .flat_map(|l| {
            let words: Vec<_> = l.split(" ").collect();
            let op = Operation::parse(words[0]).unwrap();
            let count = words[1].parse::<usize>().unwrap();
            vec![op; count]
        })
        .collect();
    
    dbg!(operations);
}

#[cfg(test)]
mod tests_part9 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_n4() {}
}
