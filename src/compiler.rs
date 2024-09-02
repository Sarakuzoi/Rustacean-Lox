use crate::{
    chunk::Chunk,
    scanner::{Scanner, TokenType},
};

pub fn compile(source: String, chunk: &Chunk) -> bool {
    let mut scanner = Scanner::init(source);
    scanner.advance();
    scanner.expression();
    scanner.consume(TokenType::EOF, "Expect end of expression.");
    // let mut line = 0;
    // loop {
    //     let token = scanner.scan_token();
    //     if token.line != line {
    //         print!("{:4} ", token.line);
    //         line = token.line;
    //     } else {
    //         print!("   | ")
    //     }
    //     // printf("%2d '%.*s'\n", token.type, token.length, token.start);
    //     print!("{:?} {}", token.r#type, token.lexeme);

    //     if token.r#type == TokenType::EOF {
    //         break;
    //     }
    // }
}
