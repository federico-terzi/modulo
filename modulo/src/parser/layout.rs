use super::split::*;
use regex::Regex;

lazy_static! {
    static ref FIELD_REGEX: Regex = Regex::new(r"\{\{(.*)\}\}").unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Text(String),  // Text
    Field(String), // id: String
}

pub fn parse_layout(layout: &str) -> Vec<Vec<Token>> {
    let mut rows = Vec::new();

    for line in layout.lines() {
        let line = line.trim();

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        let mut row: Vec<Token> = Vec::new();

        let splitter = SplitCaptures::new(&FIELD_REGEX, line);

        // Get the individual tokens
        for state in splitter {
            match state {
                SplitState::Unmatched(text) => {
                    if !text.is_empty() {
                        row.push(Token::Text(text.to_owned()))
                    }
                }
                SplitState::Captured(caps) => {
                    if let Some(name) = caps.get(1) {
                        let name = name.as_str().to_owned();
                        row.push(Token::Field(name));
                    }
                }
            }
        }

        rows.push(row);
    }

    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_layout() {
        let layout = "Hey {{name}},\nHow are you?\n  \nCheers";
        let result = parse_layout(layout);
        assert_eq!(
            result,
            vec![
                vec![
                    Token::Text("Hey ".to_owned()),
                    Token::Field("name".to_owned()),
                    Token::Text(",".to_owned())
                ],
                vec![Token::Text("How are you?".to_owned())],
                vec![Token::Text("Cheers".to_owned())],
            ]
        );
    }
}
