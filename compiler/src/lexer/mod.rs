use logos::{Logos, Span};
use std::fmt;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
#[logos(skip r"#.*")]
pub enum Token {
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("const")]
    Const,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("trait")]
    Trait,
    #[token("impl")]
    Impl,
    #[token("use")]
    Use,
    #[token("mod")]
    Mod,
    #[token("pub")]
    Pub,
    #[token("async")]
    Async,
    #[token("await")]
    Await,
    #[token("return")]
    Return,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("match")]
    Match,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("loop")]
    Loop,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("nursery")]
    Nursery,
    #[token("spawn")]
    Spawn,
    #[token("unsafe")]
    Unsafe,
    #[token("comptime")]
    Comptime,
    #[token("type")]
    Type,
    #[token("as")]
    As,
    #[token("is")]
    Is,
    #[token("in")]
    In,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token("<")]
    Lt,
    #[token("<=")]
    LtEq,
    #[token(">")]
    Gt,
    #[token(">=")]
    GtEq,
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,
    #[token("!")]
    Not,
    #[token("=")]
    Assign,
    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("*=")]
    StarEq,
    #[token("/=")]
    SlashEq,
    #[token(".")]
    Dot,
    #[token("..")]
    DotDot,
    #[token("...")]
    DotDotDot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("::")]
    ColonColon,
    #[token(";")]
    Semi,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token("|")]
    Pipe,
    #[token("?")]
    Question,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[0-9]+(_[0-9]+)*", |lex| lex.slice().replace('_', "").parse().ok())]
    Integer(u64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        let unescaped = s[1..s.len()-1].replace("\\n", "\n").replace("\\t", "\t").replace("\\\"", "\"").replace("\\\\", "\\");
        Some(unescaped)
    })]
    String(String),

    #[regex(r"'([^'\\]|\\.)*'", |lex| {
        let s = lex.slice();
        let c = &s[1..s.len()-1];
        Some(c.chars().next().unwrap())
    })]
    Char(char),

    #[regex(r"[0-9]+(_[0-9]+)*\.[0-9]+(_[0-9]+)*", |lex| lex.slice().replace('_', "").parse().ok())]
    Float(f64),

    #[token("true")]
    True,
    #[token("false")]
    False,

    #[token("_")]
    Underscore,

    Error,
}

impl Token {
    pub fn span(&self, span: Span) -> SpannedToken {
        SpannedToken {
            token: self.clone(),
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(s) => write!(f, "{}", s),
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Char(c) => write!(f, "'{}'", c),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub struct Lexer {
    lexer: logos::Lexer<'static, Token>,
    source: String,
    filename: String,
}

impl Lexer {
    pub fn new(source: &str, filename: &str) -> Self {
        let mut lexer = Token::lexer(source);
        lexer.extras = filename;
        Self {
            lexer,
            source: source.to_string(),
            filename: filename.to_string(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<SpannedToken> {
        let mut tokens = Vec::new();
        while let Some(token) = self.lexer.next() {
            let span = self.lexer.span();
            match token {
                Ok(tok) => tokens.push(tok.span(span)),
                Err(_) => {
                    let slice = &self.source[span.clone()];
                    tokens.push(SpannedToken {
                        token: Token::Error,
                        span,
                    });
                }
            }
        }
        tokens.push(SpannedToken {
            token: Token::Error,
            span: self.source.len()..self.source.len(),
        });
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("fn main() { let x = 42; }", "test.lp");
        let tokens = lexer.tokenize();
        assert!(tokens.iter().any(|t| matches!(t.token, Token::Fn)));
        assert!(tokens.iter().any(|t| matches!(t.token, Token::Let)));
        assert!(tokens.iter().any(|t| matches!(t.token, Token::Integer(42))));
    }
}