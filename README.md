# Logos `λόγος`

Create ridiculously fast Lexers.

```rust
extern crate logos;
#[macro_use]
extern crate logos_derive;

use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
enum Token {
    #[end]
    End,

    #[error]
    Error,

    #[token = "."]
    Period,

    #[regex = "[a-zA-Z]*"]
    Text,
}

fn main() {
    let mut lexer = Token::lexer("Create ridiculously fast Lexers.");

    assert_eq!(lexer.token, Token::Text);
    assert_eq!(lexer.slice(), "Create");
    assert_eq!(lexer.range(), 0..6);

    lexer.advance();

    assert_eq!(lexer.token, Token::Text);
    assert_eq!(lexer.slice(), "ridiculously");
    assert_eq!(lexer.range(), 7..19);

    lexer.advance();

    assert_eq!(lexer.token, Token::Text);
    assert_eq!(lexer.slice(), "fast");
    assert_eq!(lexer.range(), 20..24);

    lexer.advance();

    assert_eq!(lexer.token, Token::Text);
    assert_eq!(lexer.slice(), "Lexers");
    assert_eq!(lexer.range(), 25..31);

    lexer.advance();

    assert_eq!(lexer.token, Token::Period);
    assert_eq!(lexer.slice(), ".");
    assert_eq!(lexer.range(), 31..32);

    lexer.advance();

    assert_eq!(lexer.token, Token::End);
}
```

## How fast?

Ridiculously fast!

```
test logos                ... bench:       2,086 ns/iter (+/- 73) = 1021 MB/s
test logos_nul_terminated ... bench:       1,956 ns/iter (+/- 141) = 1089 MB/s
```

## License

This code is distributed under the terms of both the MIT license
and the Apache License (Version 2.0), choose whatever works for you.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
