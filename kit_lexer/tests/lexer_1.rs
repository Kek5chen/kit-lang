use kit_lexer::*;

#[test]
fn test_lexer() {
    let code = include_str!("code_files/test1.kit");
    let lexed = lex(code);

    assert_eq!(lexed.unwrap().len(), 22);
}

#[test]
fn literal_rust_source_code() {
    let code = include_str!("../src/lib.rs");
    let lexed = lex(code);

    assert!(lexed.is_ok());
}