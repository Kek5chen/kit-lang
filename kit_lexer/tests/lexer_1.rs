use kit_lexer::*;

#[test]
fn test_lexer() {
    let code = include_str!("code_files/test1.kit");
    let lexed = lex(code);

    assert_eq!(lexed.len(), 15);
}