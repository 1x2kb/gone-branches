use std::io::{BufRead, Error, Lines, StdinLock};

fn main() {
    let gone_branches = get_lines_from_stdin()
        .flat_map(parse_line_for_match)
        .reduce(|a: String, b: String| a + " " + &b)
        .unwrap_or(String::from(""));

    print!("{}", gone_branches);
}

fn get_lines_from_stdin() -> Lines<StdinLock<'static>> {
    std::io::stdin().lock().lines()
}

fn parse_line_for_match(line: Result<String, Error>) -> Option<String> {
    match line {
        Ok(line) => {
            if line.contains("[gone]") {
                let line = line.trim();
                if let Some(match_location) = line.find(|char: char| char.is_ascii_whitespace()) {
                    return Some(line.get(0..match_location).unwrap().to_string());
                }
            }
            None
        }
        Err(_) => None,
    }
}
