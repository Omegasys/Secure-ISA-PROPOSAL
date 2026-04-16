pub struct Token {
    pub text: String,
}

pub fn tokenize(line: &str) -> Vec<Token> {
    line.split_whitespace()
        .map(|s| Token { text: s.to_string() })
        .collect()
}
