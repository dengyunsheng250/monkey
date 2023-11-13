use std::vec;

#[derive(Debug, PartialEq)]
pub enum Type {
    ILLEGAL,
    EOF,
    IDENT,
    INT,

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}
#[derive(Debug)]
pub struct Token {
    t: Type,
    value: String,
}

impl Token {
    fn new(t: Type, value: String) -> Self {
        Self { t, value }
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    ch: char,
}

impl Lexer {
    fn new(s: &str) -> Self {
        Self {
            input: s.to_owned(),
            position: 0,
            ch: '\0',
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn read_char(&mut self) -> char {
        if self.position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.position).unwrap();
        }
        self.advance();
        self.ch
    }

    fn next_token(&mut self) -> Option<Token> {
        let token = match self.read_char() {
            c @ '=' => Token::new(Type::ASSIGN, String::from(c)),
            c => Token::new(Type::ILLEGAL, String::from(c)),
        };
        Some(token)
    }

    fn tokenizer(&mut self) -> Vec<Token> {
        let mut v = vec![];
        while let Some(t) = self.next_token() {
            if t.t == Type::ILLEGAL {
                break;
            }
            v.push(t);
        }
        v
    }
}

#[test]
fn testNextToken() {
    let input = "=";
    let mut lexer = Lexer::new(input);
    println!("{:?}", lexer.tokenizer());
}
