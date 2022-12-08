pub fn process(input: String) {
    let output = 
        input.split("\r\n\r\n")
            .map(|elf| {
                
                elf.lines().map(|line| {
                    
                    line.trim().parse::<u32>().unwrap()
                }).sum::<u32>()
            })
            .max()
            .unwrap();
    println!("{}", output);
}
