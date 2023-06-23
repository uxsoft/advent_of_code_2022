
#[derive(Debug)]
struct SnafuNumber {
    /// Index 0 is the least significant bit
    parts: Vec<i8>
}

impl SnafuNumber {
    fn parse_digit(i: char) -> i8 {
        match i {
            '-' => -1,
            '=' => -2,
            i => i.to_string().parse().unwrap()
        }
    }
    
    fn format_digit(i: &i8) -> char {
        match i {
            -1 => '-',
            -2 => '=',
            other => other.to_string().chars().next().unwrap()
        }
    }
    
    fn parse(input: &str) -> Self {
        let parts : Vec<i8> = input.chars().map(SnafuNumber::parse_digit).rev().collect();
        Self { parts }
    }
}

impl From<&SnafuNumber> for u128 {
    fn from(value: &SnafuNumber) -> Self {
        let mut v = 0;
        
        for i in 0..value.parts.len() {
            v += value.parts[i] as i128 * (5 as i128).pow(i as u32)
        }
        
        v as u128
    }
}

impl From<u128> for SnafuNumber {
    fn from(value: u128) -> Self {
        let mut parts = vec![]; // for carryover
        let mut value = value;
        
        while value > 0 {
            let remainder = value % 5;
            value = value / 5;
            parts.push(remainder as i8);
        }
        
        // The correct order is where index 0 is value of 5^0
        //parts.reverse();
        
        let mut carryover = 0;
        for i in 0..parts.len() {
            parts[i] += carryover;
            carryover = 0;
            
            if parts[i] > 2 {
                parts[i] = parts[i] - 5;
                carryover = 1;
            }
        }
        if carryover > 0 {
            parts.push(carryover);
        }
        
        SnafuNumber { parts }
    }
}

impl ToString for SnafuNumber {
    fn to_string(&self) -> String {
        self.parts.iter().rev().map(SnafuNumber::format_digit).collect()
    }
}

fn parse_input(input: &str) -> Vec<SnafuNumber> {
    input.lines().map(SnafuNumber::parse).collect()
}

fn part1(input: String) -> String {
    let numbers = parse_input(&input);
    
    let dec_total: u128 = numbers.iter().map(u128::from).sum();
    let snafu_total = SnafuNumber::from(dec_total);
    snafu_total.to_string()
}

fn part2(_input: String) -> String {
    "".to_owned()
}

pub fn process(input: String) {
    let result = part1(input);
    println!("Total in SNAFU: {}", result);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_u32_to_snafu() {
        println!("{}", SnafuNumber::from(1747).to_string());
        assert_eq!(SnafuNumber::from(1747).to_string(), "1=-0-2");
        assert_eq!(SnafuNumber::from(906).to_string(), "12111");
        assert_eq!(SnafuNumber::from(198).to_string(), "2=0=");
        assert_eq!(SnafuNumber::from(11).to_string(), "21");
        assert_eq!(SnafuNumber::from(201).to_string(), "2=01");
        assert_eq!(SnafuNumber::from(31).to_string(), "111");
        assert_eq!(SnafuNumber::from(1257).to_string(), "20012");
        assert_eq!(SnafuNumber::from(32).to_string(), "112");
        assert_eq!(SnafuNumber::from(353).to_string(), "1=-1=");
        assert_eq!(SnafuNumber::from(107).to_string(), "1-12");
        assert_eq!(SnafuNumber::from(7).to_string(), "12");
        assert_eq!(SnafuNumber::from(3).to_string(), "1=");
        assert_eq!(SnafuNumber::from(37).to_string(), "122");
    }
    
    #[test]
    fn test_snafu_to_u32() {
        assert_eq!(u128::from(&SnafuNumber::parse("1=-0-2")), 1747);
        assert_eq!(u128::from(&SnafuNumber::parse("12111")), 906);
        assert_eq!(u128::from(&SnafuNumber::parse("2=0=")), 198);
        assert_eq!(u128::from(&SnafuNumber::parse("21")), 11);
        assert_eq!(u128::from(&SnafuNumber::parse("2=01")), 201);
        assert_eq!(u128::from(&SnafuNumber::parse("111")), 31);
        assert_eq!(u128::from(&SnafuNumber::parse("20012")), 1257);
        assert_eq!(u128::from(&SnafuNumber::parse("112")), 32);
        assert_eq!(u128::from(&SnafuNumber::parse("1=-1=")), 353);
        assert_eq!(u128::from(&SnafuNumber::parse("1-12")), 107);
        assert_eq!(u128::from(&SnafuNumber::parse("12")), 7);
        assert_eq!(u128::from(&SnafuNumber::parse("1=")), 3);
        assert_eq!(u128::from(&SnafuNumber::parse("122")), 37);
    }

    #[test]
    fn test_example1() {
        let input = include_str!("../data/day25_dry.txt");
        let result = part1(input.to_owned());
        assert_eq!(result, "2=-1=0");
    }

    #[test]
    fn test_example2() {
        let input = include_str!("../data/day25_sharp.txt");
        let result = part1(input.to_owned());
        assert_eq!(result, "20=022=21--=2--12=-2");
    }
}