use std::ops::Range;


pub struct ElfPair {
    elf1: Range<u32>,
    elf2: Range<u32>,
}

impl ElfPair {
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> ElfPair {
        ElfPair {
            elf1: Range { start: a, end: b },
            elf2: Range { start: c, end: d },
        }
    }

    pub fn fully_contains(&self, other: &ElfPair) -> u32 {
        0
    }
}

#[derive(Debug)]
pub struct ElfRange {
    start: u32,
    end: u32,
}

impl ElfRange {
    pub fn new(start: u32, end: u32) -> ElfRange {
        ElfRange { start, end }
    }
    pub fn parse(input: &str) -> ElfRange {
        let nums = input.split("-").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        ElfRange { start: nums[0], end: nums[1] }
    }
    pub fn is_fully_enclosed_by(&self, other: &ElfRange) -> bool {
        self.start >= other.start && self.end <= other.end
    }
    pub fn is_overlapping_with(&self, other: &ElfRange) -> bool {
        let a = self.start >= other.start && self.start <= other.end;
        let b = self.end >= other.start && self.end <= other.end;
        let c = other.start >= self.start && other.start <= self.end;
        let d = other.end >= self.start && other.end <= self.end;

        a || b || c || d
    }
}

pub fn process(input: String) {
    let output = input
        .lines()
        .map(|elf| {
            let ranges = elf
                .split(",")
                .map(ElfRange::parse)
                .collect::<Vec<_>>();

            if ranges[0].is_overlapping_with(&ranges[1]) { 1 } else { 0 }
        })
        .sum::<u32>();
    println!("Assignment pair which overlaps the other: {}", output);
}

#[cfg(test)]
mod tests_part4 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_overlapping_with() {
        assert_eq!(ElfRange::new(2, 4).is_overlapping_with(&ElfRange::new(5, 6)), false);
        assert_eq!(ElfRange::new(3, 5).is_overlapping_with(&ElfRange::new(1, 2)), false);
        assert!(ElfRange::new(2,4).is_overlapping_with(&ElfRange::new(4,6)));
        assert!(ElfRange::new(4,6).is_overlapping_with(&ElfRange::new(6,9)));
        assert!(ElfRange::new(4,5).is_overlapping_with(&ElfRange::new(3,6)));
        assert!(ElfRange::new(3,6).is_overlapping_with(&ElfRange::new(4,5)));
    }
}
