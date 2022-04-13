pub fn show_matches(compare: &str, text: &str) {
    for line in text.lines() {
        if line.contains(compare) {
            println!("{}", line)
        }
    }
}
