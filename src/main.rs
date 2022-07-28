#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
enum Token {
    LParen,
    RParen,
    Plus,
    Times,
    Digits(i32),
    Id(String),
    Eof
}

#[derive(Debug)]
#[derive(Clone)]
struct Parser {
    stack: Vec<i32>,
    state_stack: Vec<usize>
}

impl Parser {
    fn new() -> Self {
        Parser {
            stack: vec![],
            state_stack: vec![0]
        }
    }

    fn get_state(&mut self) -> usize {
        return *self.state_stack.last().unwrap()
    }

    fn parse(&mut self, token: &Token) -> Option<i32> {
        'l: loop {
            let state = self.get_state();
            let reduction = match (state, token) {
                (0, Token::Eof) => 7,
                (2, Token::Plus) => 2,
                (2, Token::RParen) => 2,
                (2, Token::Eof) => 2,
                (3, Token::Plus) => 4,
                (3, Token::Times) => 4,
                (3, Token::RParen) => 4,
                (3, Token::Eof) => 4,
                (5, Token::Plus) => 6,
                (5, Token::Times) => 6,
                (5, Token::RParen) => 6,
                (5, Token::Eof) => 6,
                (9, Token::Plus) => 1,
                (9, Token::RParen) => 1,
                (9, Token::Eof) => 1,
                (10, Token::Plus) => 3,
                (10, Token::Times) => 3,
                (10, Token::RParen) => 3,
                (10, Token::Eof) => 3,
                (11, Token::Plus) => 5,
                (11, Token::Times) => 5,
                (11, Token::RParen) => 5,
                (11, Token::Eof) => 5,
                _ => break 'l
            };

            match reduction {
                1 => {
                    let x = self.stack.pop().unwrap();
                    let y = self.stack.pop().unwrap();
                    self.stack.push(x + y);
                    self.state_stack.pop();
                    self.state_stack.pop();
                    self.state_stack.pop();
                },
                2 => {
                    self.state_stack.pop();
                },
                3 => {
                    let x = self.stack.pop().unwrap();
                    let y = self.stack.pop().unwrap();
                    self.stack.push(x * y);
                    self.state_stack.pop();
                    self.state_stack.pop();
                    self.state_stack.pop();
                },
                4 => {
                    self.state_stack.pop();
                },
                5 => {
                    self.state_stack.pop();
                    self.state_stack.pop();
                    self.state_stack.pop();
                },
                6 => {
                    self.state_stack.pop();
                },
                7 => (),
                _ => unreachable!()
            }

            let state = self.get_state();
            let reduce_state = match (state, reduction) {
                (0, 1) | (0, 2) => 1,
                (0, 3) | (0, 4) => 2,
                (0, 5) | (0, 6) => 3,
                (4, 1) | (4, 2) => 8,
                (4, 3) | (4, 4) => 2,
                (4, 5) | (4, 6) => 3,
                (6, 3) | (6, 4) => 9,
                (6, 5) | (6, 6) => 3,
                (7, 5) | (7, 6) => 10,
                (0, 7) => {
                    self.stack.push(0);
                    1
                }
                _ => unreachable!()
            };
            self.state_stack.push(reduce_state);
        }

        let state = self.get_state();

        if state == 1 && *token == Token::Eof {
            return self.stack.pop()
        }

        let shift_state = match (state, token) {
            (0, Token::Digits(_)) => 5,
            (0, Token::LParen) => 4,
            (1, Token::Plus) => 6,
            (2, Token::Times) => 7,
            (4, Token::Digits(_)) => 5,
            (4, Token::LParen) => 4,
            (6, Token::Digits(_)) => 5,
            (6, Token::LParen) => 4,
            (7, Token::Digits(_)) => 5,
            (7, Token::LParen) => 4,
            (8, Token::Plus) => 6,
            (8, Token::RParen) => 11,
            (9, Token::Times) => 7,
            _ => unreachable!()
        };
        self.state_stack.push(shift_state);
        if let Token::Digits(n) = token {
            self.stack.push(*n)
        }
        None
    }
}

struct Tokenizer {
    state: usize,
    token_str: String
}

impl Tokenizer {
    fn new() -> Self {
        Tokenizer{
            state: 0,
            token_str: String::new()
        }
    }

    fn lookup(&self, c: Option<char>) -> (Option<Token>, usize, bool) {
        match (self.state, c) {
            (0, None) => (Some(Token::Eof), 0, false),
            (0, Some('(')) => (None, 1, true),
            (0, Some(')')) => (None, 2, true),
            (0, Some('+')) => (None, 3, true),
            (0, Some('*')) => (None, 4, true),
            (0, Some(c)) if c.is_ascii_digit() => (None, 5, true),
            (0, Some(c)) if c.is_ascii_alphabetic() => (None, 6, true),
            (0, Some(_)) => (None, 0, false),
            (1, _) => (Some(Token::LParen), 0, false),
            (2, _) => (Some(Token::RParen), 0, false),
            (3, _) => (Some(Token::Plus), 0, false),
            (4, _) => (Some(Token::Times), 0, false),
            (5, Some(c)) if c.is_ascii_digit() => (None, 5, true),
            (5, Some(_)) => (Some(Token::Digits(self.token_str.parse().unwrap())), 0, false),
            (5, None) => (Some(Token::Digits(self.token_str.parse().unwrap())), 0, false),
            (6, Some(c)) if c.is_ascii_alphanumeric() => (None, 6, true),
            (6,  Some(_)) => (Some(Token::Id(self.token_str.clone())), 0, false),
            (_, _) => unreachable!("")
        }
    }

    fn get_token(&mut self, c: Option<char>) -> Option<Token> {
        let (token, state, store) = self.lookup(c);
        self.state = state;
        if token.is_some() {
            self.token_str = String::new();
            let (_, state, store) = self.lookup(c);
            self.state = state;
            if let (true, Some(c)) = (store, c) {
                self.token_str.push(c)
            }
        } else if store {
            if let Some(c) = c {
                self.token_str.push(c)
            }
        }
        token
    }
}

fn main() {
    let mut tokenizer = Tokenizer::new();
    let mut parser = Parser::new();
    let mut input = String::from("2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12");
    
    loop {
        let token = loop {
            let ch = input.pop();
            if let Some(tok) = tokenizer.get_token(ch) {
                break tok
            }
        };
        match parser.parse(&token) {
            None => continue,
            Some(result) => println!("{}", result)
        }
        break
    }
}

