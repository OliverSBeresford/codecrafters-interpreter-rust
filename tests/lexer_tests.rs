use rust_interpreter::{scan, Keyword, Literal, TokenType};

#[test]
fn tokenize_print_number_semicolon() {
    let input = "print 123;";
    let tokens = scan(input);
    assert!(tokens.tokens.len() >= 4); // print, number, semicolon, EOF

    // Check individual tokens
    assert!(matches!(tokens.tokens[0].token_type, TokenType::Keyword(Keyword::Print)));
    assert!(matches!(tokens.tokens[1].token_type, TokenType::Number));
    assert!(matches!(tokens.tokens[2].token_type, TokenType::Semicolon));
    assert!(matches!(tokens.tokens.last().unwrap().token_type, TokenType::Eof));

    // Check the literal value of the number token
    assert_eq!(tokens.tokens[1].literal, Some(Literal::Number(123.0)));
}

#[test]
fn tokenize_string_literal() {
    let input = "\"hello\"\n\n";
    let tokens = scan(input);

    // Make sure it's just string, EOF
    assert_eq!(tokens.tokens.len(), 2);
    assert!(matches!(tokens.tokens[0].token_type, TokenType::String));
    assert!(matches!(tokens.tokens[1].token_type, TokenType::Eof));

    // Check the literal value
    assert_eq!(tokens.tokens[0].literal, Some(Literal::String("hello".to_string())));
}
