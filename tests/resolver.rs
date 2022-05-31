use rs_lox_tw::errors::*;
use rs_lox_tw::parser::Parser;
use rs_lox_tw::resolver::Resolver;
use rs_lox_tw::token::Token;
use rs_lox_tw::token_type::TokenType;

mod common;

#[test]
fn test_error_variable_already_exists() {
    let source = "fun bad() {
      var a = \"first\";
      var a = \"second\";
    }";

    let (mut scanner, mut interpreter) = common::scanner_and_interpreter(source);
    let mut resolver = Resolver::new(&mut interpreter);
    if let Ok(tokens) = scanner.scan_tokens() {
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(stmts) => {
                match resolver.resolve_stmts(&stmts) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{e}");
                        return;
                    }
                }

                assert_eq!(
                    interpreter.interpret(&stmts),
                    Err(LoxResult::Resolver {
                        token: Token::identifier(0, 0, 0, TokenType::Identifier, "a"),
                        error_type: ResolverErrorType::VariableAlreadyExists,
                    })
                )
            }
            Err(e) => {
                eprintln!("There was an error: {}", e)
            }
        }
    }
}
