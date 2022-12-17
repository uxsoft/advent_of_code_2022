use std::cmp;

pub fn process(input: String) {
    let field: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|i| i - 48).collect())
        .collect();

    let dim = field.len();

    let mut top_blocking : Vec<Vec<u8>> = field.clone();
    let mut top_visible : Vec<Vec<bool>> = vec![vec![true; dim]; dim];
    let mut bottom_blocking : Vec<Vec<u8>> = field.clone();
    let mut bottom_visible : Vec<Vec<bool>> = vec![vec![true; dim]; dim];
    let mut left_blocking : Vec<Vec<u8>> = field.clone();
    let mut left_visible : Vec<Vec<bool>> = vec![vec![true; dim]; dim];
    let mut right_blocking : Vec<Vec<u8>> = field.clone();
    let mut right_visible : Vec<Vec<bool>> = vec![vec![true; dim]; dim];
    
    let mut visible : Vec<Vec<bool>> = vec![vec![true; dim]; dim];

    // Compute top
    for i in 1..dim {
        for j in 0..dim {
            top_blocking[i][j] = cmp::max(top_blocking[i][j], top_blocking[i-1][j]);
            top_visible[i][j] = top_blocking[i-1][j] < field[i][j];
        }
    }

    // Compute bottom
    for i in (0..dim - 1).rev() {
        for j in 0..dim {
            bottom_blocking[i][j] = cmp::max(bottom_blocking[i][j], bottom_blocking[i+1][j]);
            bottom_visible[i][j] = bottom_blocking[i+1][j] < field[i][j];
        }
    }

    // Compute left
    for i in 0..dim {
        for j in 1..dim {
            left_blocking[i][j] = cmp::max(left_blocking[i][j], left_blocking[i][j-1]);
            left_visible[i][j] = left_blocking[i][j-1] < field[i][j];
        }
    }

    // Compute right
    for i in 0..dim {
        for j in (0..dim - 1).rev() {
            right_blocking[i][j] = cmp::max(right_blocking[i][j], right_blocking[i][j+1]);
            right_visible[i][j] = right_blocking[i][j+1] < field[i][j];
        }
    }

    // Compute 4 sizes 
    for i in 0..dim {
        for j in 0..dim {
            visible[i][j] = top_visible[i][j] || bottom_visible[i][j] || left_visible[i][j] || right_visible[i][j];
        }
    }

    let mut total_visible = 0;
    for i in 0..dim {
        for j in 0..dim {
            if visible[i][j] {
                print!("1");
                total_visible += 1;
            } else {
              print!("0");  
            }
        }
        println!();
    }
    println!("Total {} trees are visible", total_visible);
}

#[cfg(test)]
mod tests_part8 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_n4() {}
}
