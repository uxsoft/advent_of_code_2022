pub fn is_visible(i: usize, j: usize, field: &Vec<Vec<u8>>) -> bool {
    false
}

pub fn process(input: String) {
    let field: Vec<Vec<u8>> = input 
        .lines().map(|line| line.as_bytes().iter().map(|i| i - 48).collect()).collect();

    let mut visible_count = 0;
    
    for i in 1..field.len() - 2 {
        for j in 1..field[i].len() - 2 {
            if is_visible(i, j, &field) {
                visible_count += 1
            }
        }
    }
    
    println!("Counted {} visible trees", visible_count);
}

#[cfg(test)]
mod tests_part8 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_n4() {}
}
