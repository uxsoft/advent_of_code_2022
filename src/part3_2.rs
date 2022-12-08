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
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|elf| {
            
            let ha: HashSet<&u8> = elf[0].as_bytes().iter().collect();
            let hb: HashSet<&u8> = elf[1].as_bytes().iter().collect();
            let hc: HashSet<&u8> = elf[2].as_bytes().iter().collect();

            let hab: HashSet<&u8> = ha.intersection(&hb).map(|x| *x).collect();
            let common: Vec<&u8> = hab.intersection(&hc).map(|x| *x).collect();

            let prio: u32 = letter_priority(common[0]).into();
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
