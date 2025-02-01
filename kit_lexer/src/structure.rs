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
    Break,
    Continue,
    Use,
    While,
    For,
    Loop,
    Pub,
    Mod,
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
            "break" => Keyword::Break,
            "continue" => Keyword::Continue,
            "use" => Keyword::Use,
            "while" => Keyword::While,
            "for" => Keyword::For,
            "loop" => Keyword::Loop,
            "pub" => Keyword::Pub,
            "mod" => Keyword::Mod,
            _ => panic!("Unknown keyword: {}", keyword),
        }
    }
}

pub enum Expr {
    Expr(Box<Expr>),
}

pub type TokenStream = Vec<Token>;