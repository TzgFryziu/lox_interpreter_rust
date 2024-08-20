#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(String),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl Token {
    pub fn stringify(&self) -> String {
        match self {
            Token::LeftParen => format!("LEFT_PAREN ( null"),
            Token::RightParen => format!("RIGHT_PAREN ) null"),
            Token::LeftBrace => format!("LEFT_BRACE {} null", '{'),
            Token::RightBrace => format!("RIGHT_BRACE {} null", '}'),
            Token::Comma => format!("COMMA , null"),
            Token::Dot => format!("DOT . null"),
            Token::Minus => format!("MINUS - null"),
            Token::Plus => format!("PLUS + null"),
            Token::Semicolon => format!("SEMICOLON ; null"),
            Token::Star => format!("STAR * null"),
            Token::Equal => format!("EQUAL = null"),
            Token::EqualEqual => format!("EQUAL_EQUAL == null"),
            Token::Bang => format!("BANG ! null"),
            Token::BangEqual => format!("BANG_EQUAL != null"),
            Token::Less => format!("LESS < null"),
            Token::LessEqual => format!("LESS_EQUAL <= null"),
            Token::Greater => format!("GREATER > null"),
            Token::GreaterEqual => format!("GREATER_EQUAL >= null"),
            Token::Slash => format!("SLASH / null"),
            Token::EOF => format!("EOF  null"),
            Token::String(data) => format!("STRING \"{}\" {}", data, data),
            Token::Number(data) => format!("NUMBER {} {:?}", data, into_number(data)),
            Token::Identifier(data) => format!("IDENTIFIER {data} null"),
            other => format!(
                "{} {} null",
                format!("{:?}", other).to_uppercase(),
                format!("{:?}", other).to_lowercase()
            ),
        }
    }

    pub fn lexeme(&self) -> String {
        let x = self.stringify().clone();
        let v: Vec<&str> = x.split(" ").collect();
        format!("{}", v[1])
    }

    pub fn literal(&self) -> String {
        match self {
            Token::String(x) => x.to_string(),
            _ => {
                let x = self.stringify().clone();
                let v: Vec<&str> = x.split(" ").collect();
                format!("{}", v[2])
            }
        }
    }

    pub fn is_same_type(&self, other: &Token) -> bool {
        if self == other {
            return true;
        }

        matches!(
            (self, other),
            (Token::Number(_), Token::Number(_)) | (Token::String(_), Token::String(_))
        )
    }
}

fn into_number(txt: &String) -> f64 {
    txt.parse().unwrap()
}
