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
    //Identifier,
    //String,
    //Number,
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
        }
    }
}
