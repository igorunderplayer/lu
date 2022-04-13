use std::env;

fn main() {
    let match_string = env::args().nth(1).expect("bah");
    let filename = env::args().nth(2).expect("Bah2");

    let path = std::path::PathBuf::from(filename);
    let content = std::fs::read_to_string(path).expect("Nao foi possivel ler o arquivo");
    lu::show_matches(&match_string, &content);
}
