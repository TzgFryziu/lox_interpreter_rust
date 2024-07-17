mod token;
use token::Token;

pub struct Lexer {
    input: Vec<u8>,
    tokens: Vec<Token>,
    position: usize,
    read_position: usize,
    line: usize,

    pub error_code: i32,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let lex = Lexer {
            input: input.into_bytes(),
            tokens: Vec::new(),
            position: 0,
            read_position: 0,
            line: 1,
            error_code: 0,
        };

        lex
    }

    pub fn print_tokens(&mut self) {
        self.tokenize();
        for tok in self.tokens.iter() {
            println!("{}", tok.stringify())
        }
    }

    fn tokenize(&mut self) {
        while !self.is_at_end() {
            self.position = self.read_position;
            self.scan_token()
        }
        self.add_token(Token::EOF)
    }

    fn scan_token(&mut self) {
        let c = self.next_char();
        match c {
            b'(' => self.add_token(Token::LeftParen),
            b')' => self.add_token(Token::RightParen),
            b'{' => self.add_token(Token::LeftBrace),
            b'}' => self.add_token(Token::RightBrace),
            b',' => self.add_token(Token::Comma),
            b'.' => self.add_token(Token::Dot),
            b'-' => self.add_token(Token::Minus),
            b'+' => self.add_token(Token::Plus),
            b';' => self.add_token(Token::Semicolon),
            b'*' => self.add_token(Token::Star),
            b'"' => self.string(),
            b'!' => match self.match_next(b'=') {
                true => self.add_token(Token::BangEqual),
                false => self.add_token(Token::Bang),
            },
            b'=' => match self.match_next(b'=') {
                true => self.add_token(Token::EqualEqual),
                false => self.add_token(Token::Equal),
            },
            b'<' => match self.match_next(b'=') {
                true => self.add_token(Token::LessEqual),
                false => self.add_token(Token::Less),
            },
            b'>' => match self.match_next(b'=') {
                true => self.add_token(Token::GreaterEqual),
                false => self.add_token(Token::Greater),
            },
            b'/' => match self.match_next(b'/') {
                true => {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.next_char();
                    }
                }
                false => {
                    self.add_token(Token::Slash);
                }
            },

            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,
            _ if c.is_ascii_digit() => self.number(),
            _ if c.is_ascii_alphabetic() || c == b'_' => self.identifier(),
            other => self.print_error(
                format!(
                    "[line {}] Error: Unexpected character: {}",
                    self.line, other as char
                ),
                65,
            ),
        };
    }

    fn string(&mut self) {
        let starting_line = self.line;
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.next_char();
        }
        if self.is_at_end() {
            self.print_error(
                format!("[line {}] Error: Unterminated String", starting_line,),
                80,
            );
            return;
        }
        self.next_char();
        let mut value = String::new();
        for i in self.position + 1..self.read_position - 1 {
            value.push(self.input[i] as char)
        }
        self.add_token(Token::String(value));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.next_char();
        }
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.next_char();
            while self.peek().is_ascii_digit() {
                self.next_char();
            }
        }
        let mut value = String::new();

        for i in self.position..self.read_position {
            value.push(self.input[i] as char);
        }
        self.add_token(Token::Number(value))
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphabetic() || self.peek() == b'_' {
            self.next_char();
        }
        let mut value = String::new();

        for i in self.position..self.read_position {
            value.push(self.input[i] as char);
        }
        match value.as_str() {
            "and" => self.add_token(Token::And),
            "class" => self.add_token(Token::Class),
            "else" => self.add_token(Token::Else),
            "false" => self.add_token(Token::False),
            "for" => self.add_token(Token::For),
            "fun" => self.add_token(Token::Fun),
            "if" => self.add_token(Token::If),
            "nil" => self.add_token(Token::Nil),
            "or" => self.add_token(Token::Or),
            "print" => self.add_token(Token::Print),
            "return" => self.add_token(Token::Return),
            "super" => self.add_token(Token::Super),
            "this" => self.add_token(Token::This),
            "true" => self.add_token(Token::True),
            "var" => self.add_token(Token::Var),
            "while" => self.add_token(Token::While),
            _ => self.add_token(Token::Identifier(value)),
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.input[self.read_position]
    }

    fn peek_next(&self) -> u8 {
        if self.read_position + 1 >= self.input.len() {
            return b'\0';
        }
        return self.input[self.read_position + 1];
    }

    fn add_token(&mut self, tok: Token) {
        self.tokens.push(tok);
    }

    fn is_at_end(&self) -> bool {
        self.read_position >= self.input.len()
    }

    fn next_char(&mut self) -> u8 {
        let temp = self.input[self.read_position];
        self.read_position += 1;
        temp
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.input[self.read_position] != expected {
            return false;
        }
        self.read_position += 1;
        return true;
    }

    fn print_error(&mut self, message: String, errortype: i32) {
        eprintln!("{}", message);
        self.error_code = errortype;
    }
}
