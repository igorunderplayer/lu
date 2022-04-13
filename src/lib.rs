use std::{
    io::{BufReader, BufRead, self},
    fs::File,
};

pub fn show_matches(compare: &str, file: BufReader<File>) -> Result<(), io::Error> {
    for line in file.lines() {
        match line {
            Err(e) => println!("Error on line {}", e),
            Ok(ref v) => {
                if v.contains(compare) {
                   println!("{}", line?);
                }
            }
        }    
    }

    Ok(())
}
