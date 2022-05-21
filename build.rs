use std::io;
use std::process::Command;

mod generate_ast;
use generate_ast::*;

fn main() -> io::Result<()> {
    generate_ast("src")?;

    Command::new("rustfmt")
        .arg("src/expr.rs")
        .arg("src/stmt.rs")
        .spawn()
        .expect("Failed to run 'rustfmt' on build-generated files.");

    Ok(())
}
