// Token

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Special tokens
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENTIFIER,      
    INTEGER,        
    STRING,     

    // Operators
    ASSIGN,     // =
    PLUS,       // +
    MINUS,      // -
    BANG,       // !
    ASTERISK,   // *
    SLASH,      // /

    // Comparison operators
    LESS_THAN,         // <
    GREATER_THAN,         // >
    EQUAL,         // ==
    NOT_EQUAL,     // !=

    // TODO: Add regex matching patterns for these tokens below and all the misssing ones plus corresponding unit tests
    GREATER_EQUAL, // >=
    LESSER_EQUAL, // <=

    // Delimiters
    COMMA,
    SEMICOLON,
    COLON,

    LEFT_PARENTHESIS,     // (
    RIGHT_PARENTHESIS,     // )
    LEFT_BRACE,     // {
    RIGHT_BRACE,     // }
    LEFT_BRACKET,   // [
    RIGHT_BRACKET,   // ]

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    WHILE,
    FOR,
    NULL,
    UNDEFINED,

    // others
    SPACE,
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn it_should_create_token(){
        let token = Token{kind: TokenType::LET, literal:String::from("let")};

        assert_eq!(token.kind, TokenType::LET);
        assert_eq!(token.literal, "let");
    }
}