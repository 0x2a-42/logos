extern crate logos;
#[macro_use] extern crate logos_derive;

#[derive(Debug, Clone, Copy, PartialEq, Logos)]
pub enum Token {
    #[error]
    InvalidToken,

    #[end]
    EndOfProgram,

    #[regex = "[a-zA-Z$_][a-zA-Z0-9$_]*"]
    Identifier,

    #[regex = "[1-9][0-9]*"]
    Number,

    #[token = "foobar"]
    Foobar,

    #[token = "priv"]
    Priv,

    #[token = "private"]
    Private,

    #[token = "primitive"]
    Primitive,

    #[token = "protected"]
    Protected,

    #[token = "protectee"]
    Protectee,

    #[token = "in"]
    In,

    #[token = "instanceof"]
    Instanceof,

    #[token = "."]
    Accessor,

    #[token = "..."]
    Ellipsis,

    #[token = "{"]
    BraceOpen,

    #[token = "}"]
    BraceClose,

    #[token = "+"]
    OpAddition,

    #[token = "++"]
    OpIncrement,

    #[token = "="]
    OpAssign,

    #[token = "=="]
    OpEquality,

    #[token = "==="]
    OpStrictEquality,

    #[token = "=>"]
    FatArrow,
}

use logos::Logos;

fn assert_lex(source: &str, tokens: &[(Token, &str, usize, usize)]) {
    let mut lex = Token::lexer(source);

    for (token, slice, start, end) in tokens {
        assert_eq!(lex.token, *token);
        assert_eq!(lex.slice(), *slice);
        assert_eq!(lex.loc(), (*start, *end));

        lex.consume();
    }

    assert_eq!(lex.token, Token::EndOfProgram);
}

#[test]
fn empty() {
    let lex = Token::lexer("");

    assert_eq!(lex.token, Token::EndOfProgram);
    assert_eq!(lex.loc(), (0, 0));
}

#[test]
fn whitespace() {
    let lex = Token::lexer("     ");

    assert_eq!(lex.token, Token::EndOfProgram);
    assert_eq!(lex.loc(), (5, 5));
}

#[test]
fn operators() {
    assert_lex("=== == = => + ++", &[
        (Token::OpStrictEquality, "===", 0, 3),
        (Token::OpEquality, "==", 4, 6),
        (Token::OpAssign, "=", 7, 8),
        (Token::FatArrow, "=>", 9, 11),
        (Token::OpAddition, "+", 12, 13),
        (Token::OpIncrement, "++", 14, 16),
    ]);
}

#[test]
fn punctation() {
    assert_lex("{ . ... }", &[
        (Token::BraceOpen, "{", 0, 1),
        (Token::Accessor, ".", 2, 3),
        (Token::Ellipsis, "...", 4, 7),
        (Token::BraceClose, "}", 8, 9),
    ]);
}

#[test]
fn keywords() {
    assert_lex("foobar priv private primitive protected protectee in instanceof", &[
        (Token::Foobar, "foobar", 0, 6),
        (Token::Priv, "priv", 7, 11),
        (Token::Private, "private", 12, 19),
        (Token::Primitive, "primitive", 20, 29),
        (Token::Protected, "protected", 30, 39),
        (Token::Protectee, "protectee", 40, 49),
        (Token::In, "in", 50, 52),
        (Token::Instanceof, "instanceof", 53, 63),
    ]);
}

#[test]
fn numbers() {
    assert_lex("0 1 2 3 4 10 42", &[
        (Token::InvalidToken, "0", 0, 1),
        (Token::Number, "1", 2, 3),
        (Token::Number, "2", 4, 5),
        (Token::Number, "3", 6, 7),
        (Token::Number, "4", 8, 9),
        (Token::Number, "10", 10, 12),
        (Token::Number, "42", 13, 15),
    ]);
}

#[test]
fn invalid_tokens() {
    assert_lex("@-/!", &[
        (Token::InvalidToken, "@", 0, 1),
        (Token::InvalidToken, "-", 1, 2),
        (Token::InvalidToken, "/", 2, 3),
        (Token::InvalidToken, "!", 3, 4),
    ]);
}
