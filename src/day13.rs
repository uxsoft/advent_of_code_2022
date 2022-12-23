use std::{cmp::Ordering};

use nom::{sequence::{separated_pair, delimited}, multi::{separated_list1, separated_list0}, IResult, bytes::complete::tag, character::complete::newline, Parser, branch::alt};

#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),    
    Number(u32)
}

impl Packet {
    pub fn parse(input: &str) -> IResult<&str, Packet> {
        alt(
            (delimited(
            tag("["), 
            separated_list0(tag(","), Packet::parse),
            tag("]"))
                .map(|l| Packet::List(l)), 
            nom::character::complete::u32.map(|i| Packet::Number(i))
        ))(input)
    }

    pub fn cmp(&self, right: &Packet) -> Ordering {
        match (self, right) {
            (Packet::List(v1), Packet::List(v2)) => {
                for i in 0..v1.len() {
                    let l = v1.get(i);
                    let r = v2.get(i);
                    let ord = match (l, r) {
                        (None, None) => Ordering::Equal,
                        (None, Some(_)) => Ordering::Less,
                        (Some(_), None) => Ordering::Greater,
                        (Some(l), Some(r)) => l.cmp(r),
                    };
                    if ord != Ordering::Equal {
                        return ord
                    }
                }
                Ordering::Less
            },
            (Packet::List(_), Packet::Number(n)) => self.cmp(&Packet::List(vec![Packet::Number(*n)])),
            (Packet::Number(n), Packet::List(_)) => Packet::List(vec![Packet::Number(*n)]).cmp(right),
            (Packet::Number(n1), Packet::Number(n2)) => n1.cmp(n2),
        }
    }    
}

impl ToString for Packet {
    fn to_string(&self) -> String {
        match self {
            Packet::List(v) => format!("[{}]", v.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")),
            Packet::Number(n) => n.to_string(),
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(Packet::parse, newline, Packet::parse)
    )(input)
}

pub fn process(input: String) {
    let (_, packet_pairs) = parse_input(&input).unwrap();

    let mut sum = 0;
    for i in 0..packet_pairs.len() {
        let (left, right) = packet_pairs.get(i).unwrap();
        
        if left.cmp(right) != Ordering::Greater {
            println!("[{}] is in the right order", i + 1);
            sum += i + 1;
        }
    }
    println!("Sum of indices of misordered packets is: {}", sum);
    
    let mut all_packets: Vec<_> = packet_pairs
        .iter()
        .flat_map(|(l, r)| vec![l, r])
        .collect();
    let (_, d1) = Packet::parse("[[2]]").unwrap();
    let (_, d2) = Packet::parse("[[6]]").unwrap();
    all_packets.push(&d1);
    all_packets.push(&d2);
    
    all_packets.sort_by(|l, r| l.cmp(r));

    // Print sorted
    let all_packets_str = all_packets.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n");
    println!("{}", all_packets_str);

    let p1 = all_packets.iter().position(|x| x.to_string() == d1.to_string()).unwrap() + 1;
    let p2 = all_packets.iter().position(|x| x.to_string() == d2.to_string()).unwrap() + 1;

    println!("Decoder key is {} * {} = {}", p1, p2, p1 * p2);
}

#[cfg(test)]
mod tests_part13 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_compare_pair1() {
    }
}
