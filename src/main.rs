// mod ast_printer;
// use ast_printer::AstPrinter;

mod environment;

mod errors;

mod expr;
mod stmt;

mod interpreter;
use interpreter::Interpreter;

mod parser;
use parser::Parser;

mod scanner;
use scanner::*;

mod token;
mod token_type;

use std::io::{self, Write};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() || args.len() > 2 {
        eprintln!("Usage: ./rs-lox-tw [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else if let Err(e) = run_prompt() {
        eprintln!("{}", e);
    }
}

pub fn run_file(path: &str) {
    let file_content = fs::read_to_string(path).expect("Unable to read file.");
    run(file_content);
}

pub fn run_prompt() -> io::Result<()> {
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush().expect("Unable to flush stdout.");
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        if buf.is_empty() {
            continue;
        }
        run(buf);
    }
}

pub fn run(source: String) {
    let mut scanner = Scanner::new(source);
    // let printer = AstPrinter;
    let interpreter = Interpreter::new();

    if let Ok(tokens) = scanner.scan_tokens() {
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(stmts) => {
                /*
                if let Ok(printed) = printer.print(&expr) {
                    println!("AST Printer:\n{}", printed);
                } else {
                    println!("Unable to parse with LST.");
                }
                */
                match interpreter.interpret(&stmts) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e)
                    }
                }
            }
            Err(e) => {
                eprintln!("There was an error: {}", e)
            }
        }
    }
}
