use std::collections::HashSet;

fn halves(elf: &str) -> (&str, &str) {
    let half = elf.len() / 2;
    let a = &elf[..half];
    let b = &elf[half..];
    (a, b)
}

fn letter_priority(item: &u8) -> u8 {
    if *item > 96 {
        item - 96 // a = 1; z = 26
    } else {
        item - 65 + 27 // A = 27; Z = 52
    }
}

pub fn process(input: String) {
    let output: u32 = input
        .lines()
        .map(|elf| {
            let (a, b) = halves(elf);
            let ha: HashSet<_> = a.as_bytes().iter().collect();
            let hb: HashSet<_> = b.as_bytes().iter().collect();
            let both = ha.intersection(&hb);
            
            print!("---{}---", elf);

            dbg!(&ha);
            dbg!(&hb);
            dbg!(&both);
            
            let prio : u32 = both
                .map(|x| letter_priority(*x))
                .map(u32::from)
                .sum();

            println!("{} => {}", elf, prio);
                
            prio
        })
        .sum();

    println!("Sum of priorities: {}", output)
}

#[cfg(test)]
mod tests_part3 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_halves() {
        assert_eq!(halves("aabb"), ("aa", "bb"));
        assert_eq!(halves("vJrwpWtwJgWrhcsFMMfFFhFp"), ("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
    }

    #[test]
    fn test_letter_priority() {
        let letters = "pLPvts".as_bytes();
        let values: Vec<u8> = vec![16, 38, 42, 22, 20, 19];

        dbg!(letters);

        for i in 0..letters.len() {
            let letter = letters.get(i).unwrap();
            let value = values.get(i).unwrap();
            assert_eq!(letter_priority(letter), *value);
        }
    }
}
