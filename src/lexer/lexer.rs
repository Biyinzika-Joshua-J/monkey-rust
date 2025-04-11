// Token

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

#[derive(Debug)]
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

// Lexer


