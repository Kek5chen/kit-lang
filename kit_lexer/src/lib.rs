use std::sync::LazyLock;
use crate::structure::{Keyword, Token, TokenStream};
use regex::Regex;

pub mod structure;

macro_rules! make_regex {
    ($simple:expr) => {
        Regex::new(concat!(r#"^[\s]*"#, $simple)).unwrap()
    };
}

static SYMBOLS: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"(::|->|[|(){}\[\],;:!\$.&'])"#));
static OPERATORS: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"(==|>=|<=|=|<|>|\+=|-=|\*=|/=|\+|-|/|\*)"#));
static IDENT: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"([a-zA-Z_][a-zA-Z0-9_]*)"#));
static COMMENT: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"(//[^\n]*(\n|$))"#));
static KEYWORDS: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"(fn|let|struct|else|if|return|break|continue|use|while|for|loop|pub|mod)\b"#));
static LITERALS: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"(\d+|'.')"#));

// String parsing
static STRING_START: LazyLock<Regex> = LazyLock::new(|| make_regex!(r#"([rb]?(#)*")"#));

fn check_regex<'a>(regex: &Regex, haystack: &'a str) -> Option<(usize, &'a str)> {
    regex.captures(haystack).map(|m| {
        let token = m.get(1).map_or("", |m| m.as_str());
        let len = m.get(0).map_or(0, |m| m.len());
        (len, token)
    })
}

fn try_extract_string(chars: &str) -> Option<String> {
    let Some(cap) = STRING_START.captures(chars) else {
        return None;
    };

    let total_len = cap.get(1).map_or(0, |m| m.len());
    let hashes_to_end = cap.get(2).map_or(0, |m| m.len());

    let ongoing = &chars[total_len..];

    let mut is_after_quote = false;
    let mut hashes_found = 0;
    for (i, c) in ongoing.chars().enumerate() {
        match c {
            '"' => is_after_quote = true,
            '#' => {
                if is_after_quote {
                    hashes_found += 1
                }
            },
            _ => {
                is_after_quote = false;
                hashes_found = 0;
            }
        }
        if is_after_quote && hashes_found == hashes_to_end {
            return Some(chars.chars().take(total_len + i + 1 + hashes_found).collect::<String>());
        }
    }

    None
}

fn lex_next_token(chars: &str) -> Option<(usize, Token)> {
    if let Some(string) = try_extract_string(chars) {
        return Some((string.len(), Token::Literal(string)));
    }

    check_regex(&KEYWORDS, chars)
        .map(|(len, name)| (len, Token::Keyword(Keyword::from_str(name))))
        .or_else(|| {
            check_regex(&LITERALS, chars)
                .map(|(len, literal)| (len, Token::Literal(literal.to_string())))
        })
        .or_else(|| {
            check_regex(&SYMBOLS, chars)
                .map(|(len, symbol)| (len, Token::Symbol(symbol.to_string())))
        })
        .or_else(|| {
            check_regex(&IDENT, chars)
                .map(|(len, ident)| (len, Token::Ident(ident.to_string())))
        })
        .or_else(|| {
            check_regex(&COMMENT, chars)
                .map(|(len, comment)| (len, Token::Comment(comment.to_string())))
        })
        .or_else(|| {
            check_regex(&OPERATORS, chars)
                .map(|(len, operator)| (len, Token::Operator(operator.to_string())))
        })}

pub fn lex(code: &str) -> Result<TokenStream, (usize, TokenStream)> {
    let mut tokens = TokenStream::new();

    let mut current_window = code;
    while !current_window.is_empty() {
        match lex_next_token(current_window) {
            None => {
                if current_window.split_whitespace().next() == None {
                    break;
                }
                return Err((code.chars().count() - current_window.chars().count(), tokens))
            },
            Some((len, token)) => {
                current_window = &current_window[len..];
                tokens.push(token);
            }
        }
    }

    Ok(tokens)
}
