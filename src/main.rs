mod lexer;

use lexer::lexer::{Lexer};

fn main() {    
    let program  = "let x = 42;";
    
    // Lexer phase
    let mut lexer = Lexer::new(String::from(program));
    
    let tokens = lexer.getTokens();

    println!("{:#?}", tokens);
}
