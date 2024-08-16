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
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        return self.error_token("Unexpected character.".into());
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

    fn error_token(&self, message: String) -> Token {
        Token {
            r#type: TokenType::ERROR,
            lexeme: message,
            line: self.line,
        }
    }
}
