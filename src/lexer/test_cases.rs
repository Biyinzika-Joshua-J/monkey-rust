use super::token::{Token, TokenType};

pub fn get_test_cases() -> Vec<(&'static str, Vec<Token>)> {
    let test_cases = vec![
        (
            "let x = 42;",
            vec![
                Token { kind: TokenType::LET, literal: "let".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "x".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::ASSIGN, literal: "=".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::INTEGER, literal: "42".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ],
        ),
        (
            "return x + y;",
            vec![
                Token { kind: TokenType::RETURN, literal: "return".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "x".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::PLUS, literal: "+".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "y".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ],
        ),
        (
            "if (x == 10) { return true; }",
            vec![
                Token { kind: TokenType::IF, literal: "if".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::LEFT_PARENTHESIS, literal: "(".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "x".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::EQUAL, literal: "==".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::INTEGER, literal: "10".to_string() },
                Token { kind: TokenType::RIGHT_PARENTHESIS, literal: ")".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::LEFT_BRACE, literal: "{".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::RETURN, literal: "return".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::TRUE, literal: "true".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::RIGHT_BRACE, literal: "}".to_string() },
            ]
        ),
        (
            "x != y;",
            vec![
                Token { kind: TokenType::IDENTIFIER, literal: "x".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::NOT_EQUAL, literal: "!=".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "y".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ]
        ),

        (
            "while (i < 10) { i = i + 1; }",
            vec![
                Token { kind: TokenType::WHILE, literal: "while".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::LEFT_PARENTHESIS, literal: "(".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "i".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::LESS_THAN, literal: "<".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::INTEGER, literal: "10".to_string() },
                Token { kind: TokenType::RIGHT_PARENTHESIS, literal: ")".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::LEFT_BRACE, literal: "{".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "i".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::ASSIGN, literal: "=".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "i".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::PLUS, literal: "+".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::INTEGER, literal: "1".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::RIGHT_BRACE, literal: "}".to_string() },
            ]
        ),
        (
            "let y = x * 3;",
            vec![
                Token { kind: TokenType::LET, literal: "let".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "y".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::ASSIGN, literal: "=".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "x".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::ASTERISK, literal: "*".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::INTEGER, literal: "3".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ]
        ),
        /* (
            r#"let s = "hello world";"#,
            vec![
                Token { kind: TokenType::LET, literal: "let".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "s".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::ASSIGN, literal: "=".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::STRING, literal: r#""hello world""#.to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ]
        ),
        (
            "true == false;",
            vec![
                Token { kind: TokenType::TRUE, literal: "true".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::EQUAL, literal: "==".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::FALSE, literal: "false".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ]
        ),
        (
            "let arr = [1, 2, 3];",
            vec![
                Token { kind: TokenType::LET, literal: "let".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "arr".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::ASSIGN, literal: "=".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::LEFT_BRACKET, literal: "[".to_string() },
                Token { kind: TokenType::INTEGER, literal: "1".to_string() },
                Token { kind: TokenType::COMMA, literal: ",".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::INTEGER, literal: "2".to_string() },
                Token { kind: TokenType::COMMA, literal: ",".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::INTEGER, literal: "3".to_string() },
                Token { kind: TokenType::RIGHT_BRACKET, literal: "]".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ]
        ),

        (
            "fn add(a, b) { a + b; }",
            vec![
                Token { kind: TokenType::FUNCTION, literal: "fn".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "add".to_string() },
                Token { kind: TokenType::LEFT_PARENTHESIS, literal: "(".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "a".to_string() },
                Token { kind: TokenType::COMMA, literal: ",".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "b".to_string() },
                Token { kind: TokenType::RIGHT_PARENTHESIS, literal: ")".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::LEFT_BRACE, literal: "{".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "a".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::PLUS, literal: "+".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "b".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::RIGHT_BRACE, literal: "}".to_string() },
            ]
        ),
        (
            "let flag = !true;",
            vec![
                Token { kind: TokenType::LET, literal: "let".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "flag".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::ASSIGN, literal: "=".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::BANG, literal: "!".to_string() },
                Token { kind: TokenType::TRUE, literal: "true".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ]
        ),
        (
            "const PI = 3.14;",
            vec![
                Token { kind: TokenType::CONST, literal: "const".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "PI".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::ASSIGN, literal: "=".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::FLOAT, literal: "3.14".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ]
        ),
        (
            "x >= y;",
            vec![
                Token { kind: TokenType::IDENTIFIER, literal: "x".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::GREATER_THAN, literal: ">=".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::IDENTIFIER, literal: "y".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ]
        ), 
        (
            "null == undefined;",
            vec![
                Token { kind: TokenType::NULL, literal: "null".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::EQUAL, literal: "==".to_string() },
                Token { kind: TokenType::SPACE, literal: " ".to_string() },
                Token { kind: TokenType::UNDEFINED, literal: "undefined".to_string() },
                Token { kind: TokenType::SEMICOLON, literal: ";".to_string() },
            ]
        ),*/

    ];

    test_cases
}