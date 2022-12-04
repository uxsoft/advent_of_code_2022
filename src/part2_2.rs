#[derive(PartialEq, Debug, Clone, Copy)]
enum Play {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Play {
    const A: u8 = 65;

    pub fn parse(value: &u8) -> Play {
        Play::from_u8(*value - Play::A)
    }

    pub fn from_u8(value: u8) -> Play {
        match value {
            0 => Play::Rock,
            1 => Play::Paper,
            2 => Play::Scissors,
            _ => panic!("Can't <from_u8> a <Play> from {}, the value is outside of the bounds", value)
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn value(self) -> u32 {
        (self as u32) + 1
    }

    pub fn superior(self) -> Play {
        let v = (self.to_u8() + 1) % 3;
        Play::from_u8(v)
    }

    pub fn inferior(self) -> Play {
        self.superior().superior()
    }
}

#[derive(PartialEq, Debug)]
enum MatchResult {
    Lose = 0,
    Draw = 1,
    Win = 2,
}

impl MatchResult {
    const X: u8 = 88;

    pub fn parse(value: &u8) -> MatchResult {
        MatchResult::from_u8(*value - MatchResult::X)
    }

    pub fn from_u8(value: u8) -> MatchResult {
        match value {
            0 => MatchResult::Lose,
            1 => MatchResult::Draw,
            2 => MatchResult::Win,
            _ => panic!("Can't <from_u8> a <MatchResult> from {}, the value is outside of the bounds", value)
        }
    }

    pub fn to_u8(value: MatchResult) -> u8 {
        value as u8
    }

    pub fn score(&self) -> u32 {
        match self {
            MatchResult::Lose => 0,
            MatchResult::Draw => 3,
            MatchResult::Win => 6,
        }
    }
}

fn rate_transaction(opponent: Play, result: MatchResult) -> u32 {
    let your_play = match result {
        MatchResult::Lose => opponent.inferior(),
        MatchResult::Draw => opponent,
        MatchResult::Win => opponent.superior(),
    };

    your_play.value() + result.score()
}

pub fn process(input: String) {
    let output = input
        .lines()
        .map(|round| {
            let split = round.split(" ").collect::<Vec<_>>();
            let a = split.get(0).unwrap().as_bytes().get(0).unwrap();
            let a = Play::parse(a);
            let b = split.get(1).unwrap().as_bytes().get(0).unwrap();
            let b = MatchResult::parse(b);
            rate_transaction(a, b)
        })
        .sum::<u32>();

    println!("Answer: {}", output);
}

#[cfg(test)]
mod tests_part2_2 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_superior() {
        assert_eq!(Play::Rock.superior(), Play::Paper);
        assert_eq!(Play::Paper.superior(), Play::Scissors);
        assert_eq!(Play::Scissors.superior(), Play::Rock);
    }
    #[test]
    fn test_inferior() {
        assert_eq!(Play::Rock.inferior(), Play::Scissors);
        assert_eq!(Play::Paper.inferior(), Play::Rock);
        assert_eq!(Play::Scissors.inferior(), Play::Paper);
    }
    
}
