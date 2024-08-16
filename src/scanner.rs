pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    ERROR,
    EOF,
}

impl Scanner {
    pub fn init(source: String) -> Self {
        Scanner {
            source: source.chars().collect::<Vec<char>>(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.advance();

        if c.is_digit(10) {
            return self.number();
        }

        return match c {
            '(' => self.make_token(TokenType::LEFT_PAREN),
            ')' => self.make_token(TokenType::RIGHT_PAREN),
            '{' => self.make_token(TokenType::LEFT_BRACE),
            '}' => self.make_token(TokenType::RIGHT_BRACE),
            ';' => self.make_token(TokenType::SEMICOLON),
            ',' => self.make_token(TokenType::COMMA),
            '.' => self.make_token(TokenType::DOT),
            '-' => self.make_token(TokenType::MINUS),
            '+' => self.make_token(TokenType::PLUS),
            '/' => self.make_token(TokenType::SLASH),
            '*' => self.make_token(TokenType::STAR),
            '!' => {
                let aux = self.matches('=');
                return self.make_token(if aux {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                });
            }
            '=' => {
                let aux = self.matches('=');
                return self.make_token(if aux {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                });
            }
            '>' => {
                let aux = self.matches('=');
                return self.make_token(if aux {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                });
            }
            '<' => {
                let aux = self.matches('=');
                return self.make_token(if aux {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                });
            }
            '"' => self.string(),
            _ => self.error_token("Unexpected character.".into()),
        };
    }

    fn string(&mut self) -> Token {
        while self.peek() != Some(&'"') && !self.is_at_end() {
            if self.peek() == Some(&'\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.".into());
        };

        // Closing quote
        self.advance();
        self.make_token(TokenType::STRING)
    }

    fn number(&mut self) -> Token {
        while self.peek().is_some() && self.peek().unwrap().is_digit(10) {
            self.advance();
        }

        if self.peek() == Some(&'.')
            && self.peek_next().is_some()
            && self.peek_next().unwrap().is_digit(10)
        {
            self.advance();

            while self.peek().is_some() && self.peek().unwrap().is_digit(10) {
                self.advance();
            }
        }

        self.make_token(TokenType::NUMBER)
    }

    fn is_at_end(&self) -> bool {
        self.source.get(self.current).is_none()
    }

    fn make_token(&self, r#type: TokenType) -> Token {
        let mut aux = Vec::new();
        for i in self.start..self.current {
            aux.push(*self.source.get(i).unwrap());
        }
        Token {
            r#type: r#type,
            lexeme: aux.iter().collect::<String>(),
            line: self.line,
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        *self.source.get(self.current).unwrap()
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.get(self.current).unwrap() != &expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.current)
    }

    fn peek_next(&self) -> Option<&char> {
        self.source.get(self.current + 1)
    }

    fn error_token(&self, message: String) -> Token {
        Token {
            r#type: TokenType::ERROR,
            lexeme: message,
            line: self.line,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            if c.is_none() {
                return;
            }
            match c.unwrap() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                }
                '/' => {
                    if self.peek_next() == Some(&'/') {
                        while self.peek() != Some(&'\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => (),
            }
        }
    }
}
