use std::sync::LazyLock;
use crate::structure::{Keyword, Token, TokenStream};
use regex::Regex;

pub mod structure;

macro_rules! make_regex {
    ($simple:expr) => {
        Regex::new(concat!(r#"^[\s]*"#, $simple)).unwrap()
    };
}

static SYMBOLS: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"([(){},;:])"#));
static OPERATORS: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"(==|>=|<=|=|<|>)"#));
static IDENT: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"([a-zA-Z_][a-zA-Z0-9_]*)"#));
static COMMENT: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"(//[^\n]*\n)"#));
static KEYWORDS: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"(fn|let|struct|else|if|return)[\W$]"#));
static INTEGER: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"(\d+)"#));

fn check_regex<'a>(regex: &Regex, haystack: &'a str) -> Option<(usize, &'a str)> {
    regex.captures(haystack).map(|m| {
        let token = m.get(1).map_or("", |m| m.as_str());
        let len = m.get(0).map_or(0, |m| m.as_str().len());
        (len, token)
    })
}

fn lex_next_token(chars: &str) -> Option<(usize, Token)> {
    if let Some((len, name)) = check_regex(&KEYWORDS, chars) {
        Some((len, Token::Keyword(Keyword::from_str(name))))
    } else if let Some((len, symbol)) = check_regex(&SYMBOLS, chars) {
        Some((len, Token::Symbol(symbol.to_string())))
    } else if let Some((len, ident)) = check_regex(&IDENT, chars) {
        Some((len, Token::Ident(ident.to_string())))
    } else if let Some((len, comment)) = check_regex(&COMMENT, chars) {
        Some((len, Token::Comment(comment.to_string())))
    } else if let Some((len, operator)) = check_regex(&OPERATORS, chars) {
        Some((len, Token::Operator(operator.to_string())))
    } else if let Some((len, integer)) = check_regex(&INTEGER, chars) {
        Some((len, Token::Literal(integer.to_string())))
    } else {
        None
    }
}

pub fn lex(mut code: &str) -> TokenStream {
    let mut tokens = TokenStream::new();

    while !code.is_empty() {
        match lex_next_token(code) {
            None => break,
            Some((len, token)) => {
                code = &code[len..];
                tokens.push(token);
            }
        }
    }

    println!("Tokens found:");
    tokens.iter().for_each(|t| println!("  {:?}", t));

    tokens
}
