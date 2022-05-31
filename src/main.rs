use rs_lox_tw::errors::LoxResult;
use rs_lox_tw::interpreter::Interpreter;
use rs_lox_tw::parser::Parser;
use rs_lox_tw::resolver::Resolver;
use rs_lox_tw::scanner::Scanner;

use std::io::{self, BufRead, Write};
use std::{env, fs};

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    fn new() -> Self {
        Lox {
            interpreter: Interpreter::new(),
        }
    }

    fn run_file(&mut self, path: &str) -> Result<(), LoxResult> {
        let file_content = fs::read_to_string(path).expect("Unable to read file.");
        self.run(file_content)
    }

    fn run_prompt(&mut self) -> Result<(), LoxResult> {
        // Get an handle on stdin
        let stdin = io::stdin();

        // Print the prompt
        print!("> ");
        std::io::stdout().flush().expect("Unable to flush stdout.");
        for line in stdin.lock().lines() {
            // Specialy convert an IO error into a `LoxResult::IOError`
            match line {
                Ok(line) => self.run(line)?,
                Err(_) => return Err(LoxResult::IOError),
            };
            // Print the prompt
            print!("> ");
            std::io::stdout().flush().expect("Unable to flush stdout.");
        }

        Ok(())
    }

    fn run(&mut self, source: String) -> Result<(), LoxResult> {
        let mut scanner = Scanner::new(&source);

        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);

        let statements = parser.parse()?;
        let mut resolver = Resolver::new(&mut self.interpreter);
        resolver.resolve_stmts(&statements)?;

        self.interpreter.interpret(&statements)?;
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();

    if args.is_empty() || args.len() > 2 {
        eprintln!("Usage: ./rs-lox-tw [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        if let Err(e) = lox.run_file(&args[1]) {
            eprintln!("{}", e);
        }
    } else if let Err(e) = lox.run_prompt() {
        eprintln!("{}", e);
    }
}
