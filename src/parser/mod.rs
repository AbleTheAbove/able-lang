mod item;
mod utils;

use item::Expr;

use crate::error::{Error, ErrorKind};
use crate::tokens::Token;

use logos::Logos;

/// Parser structure / state machine
pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
    ast: Vec<Expr>,
}

impl<'a> Parser<'a> {
    /// Create a new parser object
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Token::lexer(source),
            ast: Vec::new(),
        }
    }

    pub fn init(&mut self) -> Result<Vec<Expr>, Error> {
        loop {
            let token = self.lexer.next();
            if token.is_none() {
                return Ok(self.ast.clone());
            };
            let expr = self.parse_expr(&token)?;
            self.ast.push(expr);
        }
    }

    fn parse_expr(&mut self, token: &Option<Token>) -> Result<Expr, Error> {
        if matches!(token, None) {
            return Err(Error {
                kind: ErrorKind::EndOfTokenStream,
                position: self.lexer.span(),
            });
        }

        Ok(todo!())
    }

    /*
    /// Start parsing Token Vector into Abstract Syntax Tree
    pub fn parse(&mut self) -> Vec<Expr> {
        let mut ast = vec![];
        while let Some(token) = self.lexer.next() {
            let expr = match token {
                Token::Variable => self.variable_declaration(),
                Token::Function => self.function_declaration(),
                Token::BfFunction => self.bff_declaration(),
                Token::RightBrace => return ast,
                _ => Err(Error {
                    kind: ErrorKind::SyntaxError,
                    position: 0..0,
                }),
            };
            match expr {
                Ok(o) => ast.push(o),
                Err(e) => {
                    e.panic(self.lexer.slice());
                    break;
                }
            }
        }

        ast
    }
    */

    /// Parse variable declaration
    ///
    /// `var [iden] = [literal];`
    fn variable_declaration(&mut self) -> Result<Expr, Error> {
        let iden = self.require(Token::Identifier)?;

        let init = match self.lexer.next() {
            Some(Token::Semicolon) => None,
            Some(Token::Assignment) => {
                let value = self.require(Token::Boolean)?; // TODO: Shouldn't be limited to boolean (pattern match?)
                self.require(Token::Semicolon)?;
                Some(value)
            }
            _ => {
                return Err(Error {
                    kind: ErrorKind::SyntaxError("Unexpected token".to_owned()),
                    position: self.lexer.span(),
                })
            }
        };

        Ok(Expr::VariableDeclaration { iden, init })
    }

    /// Declare function
    ///
    /// `functio [iden] ([expr], [expr]) { ... }
    fn function_declaration(&mut self) -> Result<Expr, Error> {
        let iden = self.require(Token::Identifier)?;
        self.require(Token::LeftParenthesis)?;
        // TODO: Arguments
        self.require(Token::RightParenthesis)?;
        self.require(Token::LeftBrace)?;
        let body = vec![];

        Ok(Expr::FunctionDeclaration { iden, body })
    }

    /// Declare BF FFI Function
    ///
    /// `bff [iden] { ... }`
    fn bff_declaration(&mut self) -> Result<Expr, Error> {
        let iden = self.require(Token::Identifier)?;
        self.require(Token::LeftBrace)?;
        let mut code = String::new();
        while let Some(token) = self.lexer.next() {
            code.push_str(match token {
                Token::OpGt
                | Token::OpLt
                | Token::Addition
                | Token::Subtract
                | Token::FullStop
                | Token::Comma
                | Token::LeftBracket
                | Token::RightBracket => self.lexer.slice(),
                Token::RightBrace => break,
                _ => break,
            });
        }
        self.require(Token::RightBrace)?;
        Ok(Expr::BfFDeclaration { iden, code })
    }
}
