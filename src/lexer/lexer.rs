use regex::Regex;
use super::token::{Token, TokenType};

// Lexer
const token_patterns: [(&str, &str); 36] = [
    // Whitespace
    (r"^\s+", "SPACE"),

    // Keywords
    (r"^\bfunction\b", "FUNCTION"),
    (r"^\blet\b", "LET"),
    (r"^\btrue\b", "TRUE"),
    (r"^\bfalse\b", "FALSE"),
    (r"^\bif\b", "IF"),
    (r"^\belse\b", "ELSE"),
    (r"^\breturn\b", "RETURN"),
    (r"^\bwhile\b", "WHILE"),
    (r"^\bfor\b", "FOR"),
    (r"^\bnull\b", "NULL"),
    (r"^\bundefined\b", "UNDEFINED"),

    // Identifiers
    (r"^[a-zA-Z_][a-zA-Z0-9_]*", "IDENTIFIER"),

    // Literals
    (r#"^"([^"\\]|\\.)*""#, "STRING"), // double quotes
    // TODO: Support single quote strings
    (r"^\d+", "INTEGER"),

    // Comparison Operators
    (r"^==", "EQUAL"),
    (r"^!=", "NOT_EQUAL"),
    (r"^<", "LESS_THAN"),
    (r"^>", "GREATER_THAN"),

    // Operators
    (r"^=", "ASSIGN"),
    (r"^\+", "PLUS"),
    (r"^-", "MINUS"),
    (r"^!", "BANG"),
    (r"^\*", "ASTERISK"),
    (r"^/", "SLASH"),

    // Delimiters
    (r"^,", "COMMA"),
    (r"^;", "SEMICOLON"),
    (r"^:", "COLON"),

    (r"^\(", "LEFT_PARENTHESIS"),
    (r"^\)", "RIGHT_PARENTHESIS"),
    (r"^\{", "LEFT_BRACE"),
    (r"^\}", "RIGHT_BRACE"),
    (r"^\[", "LEFT_BRACKET"),
    (r"^\]", "RIGHT_BRACKET"),

     // Special tokens
     (r"^$", "EOF"),
     (r".", "ILLEGAL"),
];


#[derive(Debug)]
pub struct Lexer {
    program: String,
    tokens: Vec<Token>,
    current_token_idx: usize,
    program_length: usize,
}

impl Lexer {
    pub fn new(program: String) -> Self {
        let program_length = program.len();

        Lexer { 
                program, 
                tokens: vec![], 
                current_token_idx: 0, 
                program_length,
            }
    }

    fn generateTokens(&mut self) {

       while self.current_token_idx < self.program_length {
            let input_slice = &self.program[self.current_token_idx..self.program_length];
            
           for (regex_str, token_type) in token_patterns {
               let re = Regex::new(regex_str).unwrap();

               if let Some(matched) = re.find(input_slice) {
                    let matched_str = matched.as_str();
                    self.current_token_idx += matched_str.len();
                    
                    let token = Token{
                        kind: lookup_token_type(token_type),
                        literal: matched_str.to_string()
                    };

                    // println!("{:#?}", token);

                    self.tokens.push(token);

                    break;
                } else {
                    // TODO: handle the case where there is no match. Probably do nothing.
                    // println!("No match");
                }

           }
       }
    }

    pub fn getTokens(&mut self) -> &Vec<Token>{
        self.generateTokens();

        &self.tokens
    }

}

fn lookup_token_type(token_type: &str) -> TokenType {
    match token_type {
        "EOF" => TokenType::EOF,
        "ILLEGAL" => TokenType::ILLEGAL,
        "IDENTIFIER" => TokenType::IDENTIFIER,
        "INTEGER" => TokenType::INTEGER,
        "STRING" => TokenType::STRING,
        "ASSIGN" => TokenType::ASSIGN,
        "PLUS" => TokenType::PLUS,
        "MINUS" => TokenType::MINUS,
        "BANG" => TokenType::BANG,
        "ASTERISK" => TokenType::ASTERISK,
        "SLASH" => TokenType::SLASH,
        "LESS_THAN" => TokenType::LESS_THAN,
        "GREATER_THAN" => TokenType::GREATER_THAN,
        "EQUAL" => TokenType::EQUAL,
        "NOT_EQUAL" => TokenType::NOT_EQUAL,
        "COMMA" => TokenType::COMMA,
        "SEMICOLON" => TokenType::SEMICOLON,
        "COLON" => TokenType::COLON,
        "LEFT_PARENTHESIS" => TokenType::LEFT_PARENTHESIS,
        "RIGHT_PARENTHESIS" => TokenType::RIGHT_PARENTHESIS,
        "LEFT_BRACE" => TokenType::LEFT_BRACE,
        "RIGHT_BRACE" => TokenType::RIGHT_BRACE,
        "LEFT_BRACKET" => TokenType::LEFT_BRACKET,
        "RIGHT_BRACKET" => TokenType::RIGHT_BRACKET,
        "FUNCTION" => TokenType::FUNCTION,
        "LET" => TokenType::LET,
        "TRUE" => TokenType::TRUE,
        "FALSE" => TokenType::FALSE,
        "IF" => TokenType::IF,
        "ELSE" => TokenType::ELSE,
        "RETURN" => TokenType::RETURN,
        "WHILE" => TokenType::WHILE,
        "FOR" => TokenType::FOR,
        "NULL" => TokenType::NULL,
        "UNDEFINED" => TokenType::UNDEFINED,
        "SPACE" => TokenType::SPACE,
        _ => TokenType::ILLEGAL,
    }
}


#[cfg(test)]
mod test {

    use crate::lexer::test_cases;

    use super::*;

    #[test]
    fn it_should_init_lexer(){
        let program  = "let x = 42;";
        let mut lexer = Lexer::new(String::from(program));

        assert_eq!(lexer.program, program);
        assert_eq!(lexer.current_token_idx, 0);
        assert_eq!(lexer.program_length, program.len());
        assert_eq!(lexer.tokens.len(), 0);
    }

    fn assert_tokens(input: &str, expected: &Vec<Token>) {
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.getTokens();
    
        assert_eq!(tokens, expected);
    }
    
    #[test]
    fn it_should_create_tokens() {
       
       let test_cases = test_cases::get_test_cases();
    
        for (input, expected_tokens) in test_cases.iter() {
            assert_tokens(input, expected_tokens);
        }

    }
}