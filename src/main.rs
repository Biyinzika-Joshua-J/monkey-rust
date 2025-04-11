mod lexer;

use lexer::lexer::{Token, TokenType};

fn main() {
    let token = Token{kind: TokenType::IDENTIFIER, literal:String::from("name")};
    println!("{:#?}", token);
}
