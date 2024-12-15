use crate::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

pub struct Compiler {
    scanner: Scanner,
    parser: Parser,
    compiling_chunk: Chunk,
}

#[derive(Default)]
struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scanner: Scanner::init(String::new()),
            parser: Parser::default(),
            compiling_chunk: Chunk::new(),
        }
    }

    pub fn compile(&mut self, source: String, chunk: &Chunk) -> bool {
        self.scanner = Scanner::init(source);
        self.compiling_chunk = chunk.clone();
        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression.");
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
        self.end_compiler();
        !self.parser.had_error
    }

    fn expression(&mut self) {
        todo!();
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.r#type != TokenType::ERROR {
                break;
            }
            self.error_at_current(self.parser.current.lexeme.clone());
        }
    }

    fn consume(&mut self, r#type: TokenType, message: &str) {
        if self.parser.current.r#type == r#type {
            self.advance();
            return;
        }

        self.error_at_current(message.to_string());
    }

    fn emit_byte(&mut self, byte: u8) {
        self.compiling_chunk
            .write(byte, Some(self.parser.previous.line));
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OP_RETURN as u8);
    }

    fn make_constant(&mut self, value: Value) -> usize {
        let constant = self.compiling_chunk.add_constant(value);
        if constant <= 0xff {
            self.emit_bytes(OpCode::OP_CONSTANT as u8, constant as u8);
        } else {
            self.emit_bytes(OpCode::OP_CONSTANT_LONG as u8, constant as u8);
            self.emit_byte((constant >> 8) as u8);
            self.emit_byte((constant >> 16) as u8);
        }
        constant
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        if constant <= 0xff {
            self.emit_bytes(OpCode::OP_CONSTANT as u8, constant as u8);
        } else {
            self.emit_bytes(OpCode::OP_CONSTANT_LONG as u8, constant as u8);
            self.emit_byte((constant >> 8) as u8);
            self.emit_byte((constant >> 16) as u8);
        }
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn number(&mut self) {
        let value = self.parser.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(value);
    }

    fn error_at_current(&mut self, message: String) {
        self.error_at(&self.parser.current.clone(), message);
    }

    fn error(&mut self, message: String) {
        self.error_at(&self.parser.previous.clone(), message);
    }

    fn error_at(&mut self, token: &Token, message: String) {
        if self.parser.panic_mode {
            return;
        }

        self.parser.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        if token.r#type == TokenType::EOF {
            eprint!(" at end");
        } else if token.r#type == TokenType::ERROR {
            // Nothing
        } else {
            eprint!(" at '{}'", token.lexeme);
        }

        eprintln!(": {}", message);
        self.parser.had_error = true;
    }
}
