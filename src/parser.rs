pub mod expr;
mod printer;

use super::token::Token;
use expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use printer::AstPrinter;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Expr {
        return self.expression();
    }

    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(vec![Token::BangEqual, Token::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(Binary {
                left: expr,
                operator,
                right,
            }));
        }
        expr
    }
    

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(vec![
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(Binary {
                left: expr,
                operator,
                right,
            }));
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(vec![Token::Minus, Token::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(Binary {
                left: expr,
                operator,
                right,
            }));
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(vec![Token::Slash, Token::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(Binary {
                left: expr,
                operator,
                right,
            }));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(vec![Token::Bang, Token::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(Box::new(Unary { operator, right }));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(vec![Token::False]) {
            return Expr::Literal(Box::new(Literal {
                value: "false".to_string(),
            }));
        }
        if self.match_tokens(vec![Token::True]) {
            return Expr::Literal(Box::new(Literal {
                value: "true".to_string(),
            }));
        }
        if self.match_tokens(vec![Token::Nil]) {
            return Expr::Literal(Box::new(Literal {
                value: "null".to_string(),
            }));
        }

        if self.match_tokens(vec![
            Token::Number("0".to_string()),
            Token::String("".to_string()),
        ]) {
            return Expr::Literal(Box::new(Literal {
                value: self.previous().literal(),
            }));
        }

        if self.match_tokens(vec![Token::LeftParen]) {
            let expr = self.expression();
            self.consume(
                Token::RightParen,
                "Expect ')' after expression.".to_string(),
            );
            return Expr::Grouping(Box::new(Grouping { expression: expr }));
        }

        self.print_err(self.peek().clone(), "Expect expression.".to_string());

        Expr::Literal(Box::new(Literal {
            value: "bad".to_string(),
        }))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous() == Token::Semicolon {
                return;
            }
            match self.peek() {
                Token::Class
                | Token::Fun
                | Token::Var
                | Token::For
                | Token::If
                | Token::While
                | Token::Print
                | Token::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn consume(&mut self, t: Token, message: String) -> Token {
        if self.check(&t) {
            return self.advance();
        }
        self.print_err(t, message);
        Token::And
    }
    fn print_err(&self, t: Token, m: String) {
        eprintln!("{:?},{}", t, m);
        panic!()
    }

    fn match_tokens(&mut self, types: Vec<Token>) -> bool {
        for t in types.iter() {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, t: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().is_same_type(t)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    fn is_at_end(&self) -> bool {
        match self.tokens[self.current] {
            Token::EOF => return true,
            _ => return false,
        }
    }
    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }
    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }
}

pub fn test(tokens: Vec<Token>) {
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();

    let mut printer = AstPrinter;
    println!("{}", printer.print(expr));
}
