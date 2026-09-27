use lalrpop_util::lalrpop_mod;
use std::path::PathBuf;

lalrpop_mod!(pub grammar);

use crate::{
    ast::{AstNode, Expr, Item, Pattern, Stmt, Type},
    lexer::{SpannedToken, Token},
    diagnostics::{DiagnosticBag, Label},
};

pub struct Parser {
    tokens: Vec<SpannedToken>,
    filename: String,
    diagnostics: DiagnosticBag,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>, filename: &str) -> Self {
        Self {
            tokens,
            filename: filename.to_string(),
            diagnostics: DiagnosticBag::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<AstNode<Item>> {
        let parser = grammar::ProgramParser::new();
        let tokens: Vec<_> = self.tokens.iter().map(|t| (t.token.clone(), t.span.clone())).collect();
        
        match parser.parse(self, tokens) {
            Ok(ast) => ast,
            Err(e) => {
                self.diagnostics.push(Label::error(e));
                Vec::new()
            }
        }
    }

    pub fn diagnostics(&self) -> &DiagnosticBag {
        &self.diagnostics
    }

    pub fn take_diagnostics(self) -> DiagnosticBag {
        self.diagnostics
    }
}

impl lalrpop_util::ParseError<usize, Token, &'static str> {
    fn to_diagnostic(&self, filename: &str, source: &str) -> Label {
        use lalrpop_util::ParseError::*;
        match self {
            InvalidToken { location } => {
                let span = *location..*location + 1;
                Label::error(format!("Unexpected token at {}", location))
                    .with_span(span)
            }
            UnrecognizedEOF { location, expected } => {
                let span = *location..*location;
                Label::error(format!("Unexpected EOF, expected: {}", expected.join(", ")))
                    .with_span(span)
            }
            UnrecognizedToken { token, expected } => {
                let (start, end) = token;
                Label::error(format!("Unrecognized token {:?}, expected: {}", token.1, expected.join(", ")))
                    .with_span(*start..*end)
            }
            ExtraToken { token } => {
                let (start, end) = token;
                Label::error(format!("Extra token {:?}", token.1))
                    .with_span(*start..*end)
            }
            User { error } => Label::error(*error),
        }
    }
}

impl<'input> lalrpop_util::Parser<'input> for Parser {
    type Token = Token;
    type Error = lalrpop_util::ParseError<usize, Token, &'static str>;

    fn next_token(&mut self) -> Option<Result<Self::Token, Self::Error>> {
        if let Some(token) = self.tokens.pop() {
            Some(Ok(token.token))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(source: &str) -> Vec<AstNode<Item>> {
        let mut lexer = Lexer::new(source, "test.lp");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens, "test.lp");
        parser.parse()
    }

    #[test]
    fn test_parse_function() {
        let ast = parse("fn main() { }");
        assert_eq!(ast.len(), 1);
        match &ast[0].node {
            Item::Fn(f) => assert_eq!(f.name, "main"),
            _ => panic!("Expected function"),
        }
    }

    #[test]
    fn test_parse_let() {
        let ast = parse("let x = 42;");
        assert_eq!(ast.len(), 1);
        match &ast[0].node {
            Item::Stmt(Stmt::Let { name, .. }) => assert_eq!(name, "x"),
            _ => panic!("Expected let statement"),
        }
    }
}