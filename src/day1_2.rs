pub fn process(input: String) {
    let mut elfs : Vec<u32> = 
        input.split("\r\n\r\n")
            .map(|elf| {
                
                elf.lines().map(|line| {
                    
                    line.trim().parse::<u32>().unwrap()
                }).sum::<u32>()
            })
            .collect();

    elfs.sort();
    elfs.reverse();
    
    let output = elfs[0] + elfs[1] + elfs[2];

    println!("{}", output);
}
