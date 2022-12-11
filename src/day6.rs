use std::collections::LinkedList;

pub fn index_of_unique_chars(n: usize, input: &str) -> usize {
    let chars = input.as_bytes();
    let mut buffer: LinkedList<u8> = LinkedList::new(); // Last n unique characters
    let mut i = 0;
    let mut n_unique = 0;

    loop {
        let current_char = chars.get(i).unwrap();

        // If we encounter a duplicate keep removing until we remove the duplicate
        while buffer.contains(current_char) {
            // Reduce buffer by 1 char from the back
            buffer.pop_back();
            n_unique = n_unique - 1;
        } 
        
        // Add current char
        n_unique = n_unique + 1;
        buffer.push_front(*current_char);

        
        
        // if buffer.len() > n {
        //     buffer.pop_back();
        // }

        // If we have 4 unique chars we succeeded and we can return
        // I has index of the last character, we need to subtract 4?
        // If not move on
        {
            // DEBUG
            let processed_str : &str = &input.to_string()[0..i+1];
            println!("[{}] {} {}", i, n_unique, processed_str);   
        }
        if n_unique == n {
            break (i + 1);
        } 
        i = i + 1;
    }
}

pub fn process(input: String) {
    let i = index_of_unique_chars(4, input.as_str());
    println!("First 4 unique chars appear at {}", i);

    let j = index_of_unique_chars(14, input.as_str());
    println!("First 14 unique chars appear at {}", j);
}

#[cfg(test)]
mod tests_part6 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_n4() {
        assert_eq!(
            index_of_unique_chars(4, "mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            7
        );
        assert_eq!(
            index_of_unique_chars(4, "nppdvjthqldpwncqszvftbrmjlhg"), 
            6
        );
        assert_eq!(
            index_of_unique_chars(4, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
        assert_eq!(
            index_of_unique_chars(4, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            11
        );
        assert_eq!(
            index_of_unique_chars(4, "bvwbjplbgvbhsrlpgdmjqwftvncz"), 
            5
        );
    }
    fn test_n14() {
        assert_eq!(
            index_of_unique_chars(4, "mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            19
        );
        assert_eq!(
            index_of_unique_chars(4, "nppdvjthqldpwncqszvftbrmjlhg"), 
            23
        );
        assert_eq!(
            index_of_unique_chars(4, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            23
        );
        assert_eq!(
            index_of_unique_chars(4, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            29
        );
        assert_eq!(
            index_of_unique_chars(4, "bvwbjplbgvbhsrlpgdmjqwftvncz"), 
            26
        );
    }
}
