use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;

enum WindDirection {
    Left,
    Right
}

impl WindDirection {
    pub fn parse(input: &str) -> IResult<&str, WindDirection> {
        alt((
            map(tag(">"), |_| WindDirection::Right),
            map(tag("<"), |_| WindDirection::Left),
        ))(input)
    }
}

struct Wind {
    pattern: Vec<WindDirection>,
    i: usize
}

impl Wind {
    pub fn parse(input: &str) -> IResult<&str, Wind> {
        map(
            many1(WindDirection::parse),
            |p| Wind { i: 0, pattern: p }
        )(input)
    }
    pub fn next(&mut self) -> &WindDirection {
        let w = self.pattern.get(self.i).expect("Wind is broken, i should always in bounds of the pattern");
        self.i = (self.i + 1) % self.pattern.len();
        w
    }
}

trait Rock {
    
}



pub fn process(input: String) {
    let (_, wind) = Wind::parse(&input).unwrap();
    
    
}