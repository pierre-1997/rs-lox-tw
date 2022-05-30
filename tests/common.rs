use rs_lox_tw::interpreter::Interpreter;
use rs_lox_tw::scanner::Scanner;

pub fn scanner_and_interpreter(source: &str) -> (Scanner, Interpreter) {
    let scanner = Scanner::new(source);
    let interpreter = Interpreter::new();

    (scanner, interpreter)
}
