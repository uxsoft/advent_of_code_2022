pub fn process(input: String) {
    let field: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|i| i - 48).collect())
        .collect();

    let dim = field.len();

    let mut top_scenic_score : u32 = 0;
    

    // Compute top
    for i in 1..(dim-1) {
        for j in 1..(dim-1) {
            let tree_house = field[i][j];

            let mut count_up = 0;
            for k in (0..i).rev() { // i-1=..=0
                count_up += 1;
                if field[k][j] >= tree_house {
                    break;
                }
            }
            
            let mut count_down = 0; // (i+1)=..dim
            for k in (i + 1)..dim {
                count_down += 1;
                if field[k][j] >= tree_house {
                    break;
                }
            }
            
            let mut count_left = 0; // j-1=..=0
            for k in (0..j).rev() {
                count_left += 1;
                if field[i][k] >= tree_house {
                    break;
                }
            }
            
            let mut count_right = 0; // j+1=..dim
            for k in (j + 1)..dim {
                count_right += 1;
                if field[i][k] >= tree_house {
                    break;
                }
            }
            
            let scenic_score = count_up * count_down * count_left * count_right;
            if top_scenic_score < scenic_score {
                top_scenic_score = scenic_score;
                println!("Found a tree of height {} located at [x={}, y={}] with score {} (up={} * left={} * right={} * bottom={})", tree_house, j, i, scenic_score, count_up, count_left, count_right, count_down);
            }
        }
    }


    println!("The top scenic score is {}", top_scenic_score);
}

#[cfg(test)]
mod tests_part8 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_n4() {}
}
