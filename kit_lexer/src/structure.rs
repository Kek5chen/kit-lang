#[derive(Debug)]
pub enum Token {
    Ident(String),
    Keyword(Keyword),
    Operator(String),
    Symbol(String),
    Literal(String),
    Comment(String),
}

#[derive(Debug)]
pub enum Keyword {
    Fn,
    Let,
    Struct,
    ElseIf,
    Else,
    If,
    Return,
}

impl Keyword {
    pub fn from_str(keyword: &str) -> Keyword {
        match keyword {
            "fn" => Keyword::Fn,
            "let" => Keyword::Let,
            "struct" => Keyword::Struct,
            "else if" => Keyword::ElseIf,
            "else" => Keyword::Else,
            "if" => Keyword::If,
            "return" => Keyword::Return,
            _ => panic!("Unknown keyword: {}", keyword),
        }
    }
}

pub enum Expr {
    Expr(Box<Expr>),
}

pub type TokenStream = Vec<Token>;