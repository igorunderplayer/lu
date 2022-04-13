use std::{
    io::BufReader,
    fs::File,
    path,
    env
};

fn main() -> std::io::Result<()> {
    let match_string = env::args().nth(1).expect("bah");
    let filename = env::args().nth(2).expect("Bah2");

    let path = path::PathBuf::from(filename);
    let content = BufReader::new(File::open(path).expect("COISO ERRADO"));

    lu::show_matches(&match_string, content)?;

    Ok(())
}
