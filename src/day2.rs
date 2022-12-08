const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;
const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSS: u32 = 0;

const A: u8 = 64;
const X: u8 = 87;

fn rate_transaction(opponent: &u8, you: &u8) -> u32 {
    let opponent: u32 = (opponent - A).into();
    let you: u32 = (you - X).into();

    assert!(opponent < 4);
    assert!(you < 4);
    print!("Match {} vs {} ", opponent, you);

    if opponent == you {
        println!("DRAW");
        you + DRAW
    } else if opponent == (if you == (ROCK as u32) {3} else {you-1}) {
        // Rock=1 < Paper=2
        // Paper=2 < Scissors=3
        // Scissors=3 < Rock=1
        println!("WIN");
        you + WIN
    } else {
        println!("LOSS");
        you + LOSS
    }
}

pub fn process(input: String) {
    let output = input
        .lines()
        .map(|round| {
            let split = round.split(" ").collect::<Vec<_>>();
            let a = split.get(0).unwrap().as_bytes().get(0).unwrap();
            let b = split.get(1).unwrap().as_bytes().get(0).unwrap();
            rate_transaction(a, b)
        })
        .sum::<u32>();

    println!("Answer: {}", output);
}


#[cfg(test)]
mod tests_part2 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_loss_rock() {
        assert_eq!(
            rate_transaction(&(A + PAPER), &(X + ROCK)),
            LOSS + (ROCK as u32)
        );
    }
    #[test]
    fn test_loss_paper() {
        assert_eq!(
            rate_transaction(&(A + SCISSORS), &(X + PAPER)),
            LOSS + (PAPER as u32)
        );
    }
    #[test]
    fn test_loss_scissors() {
        assert_eq!(
            rate_transaction(&(A + ROCK), &(X + SCISSORS)),
            LOSS + (SCISSORS as u32)
        );
    }

    #[test]
    fn test_draw_paper() {
        assert_eq!(
            rate_transaction(&(A + PAPER), &(X + PAPER)),
            DRAW + (PAPER as u32)
        );
    }

    #[test]
    fn test_win_rock() {
        assert_eq!(
            rate_transaction(&(A + SCISSORS), &(X + ROCK)),
            WIN + (ROCK as u32)
        );
    }
    #[test]
    fn test_win_paper() {
        assert_eq!(
            rate_transaction(&(A + ROCK), &(X + PAPER)),
            WIN + (PAPER as u32)
        );
    }
    #[test]
    fn test_win_scissors() {
        assert_eq!(
            rate_transaction(&(A + PAPER), &(X + SCISSORS)),
            WIN + (SCISSORS as u32)
        );
    }
}
