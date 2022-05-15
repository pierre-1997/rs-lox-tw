use std::fs::File;
use std::io::prelude::*;

pub fn generate_ast(output_dir: &str) -> std::io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        vec![
            "Assign   : Token name, Box<Expr> value".to_string(),
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Unary    : Token operator, Box<Expr> right".to_string(),
            "Grouping : Box<Expr> expression".to_string(),
            "Literal  : Option<Object> value".to_string(),
            "Variable : Token name".to_string(),
        ],
    )?;

    define_ast(
        output_dir,
        "Stmt",
        vec![
            "Block      : Vec<Stmt> statements".to_string(),
            "Expression : Expr expression".to_string(),
            "Print      : Expr expression".to_string(),
            "Var        : Token name, Option<Expr> initializer".to_string(),
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(output_dir.to_owned() + "/" + &base_name.to_lowercase() + ".rs")?;

    // Imports
    file.write_all(b"use crate::errors::LoxError;\n")?;
    if base_name == "Stmt" {
        file.write_all(b"use crate::expr::Expr;\n")?;
        file.write_all(b"use crate::token::Token;\n")?;
    } else if base_name == "Expr" {
        file.write_all(b"use crate::token::{Object, Token};\n")?;
    }
    // Additional '\n' after imports
    file.write_all(b"\n")?;

    // Define Expr enum
    file.write_all(format!("#[derive(Debug)]\npub enum {} {{\n", base_name).as_bytes())?;
    for ttype in &types
        .iter()
        .map(|s| s.split(':').collect::<Vec<&str>>()[0].trim())
        .collect::<Vec<&str>>()
    {
        file.write_all(format!("    {}({}{}),\n", ttype, ttype, base_name).as_bytes())?;
    }
    file.write_all(b"}\n\n")?;

    file.write_all(format!("impl {} {{\n", base_name).as_bytes())?;
    file.write_all(
        format!(
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{\n",
            base_name
        )
        .as_bytes(),
    )?;
    file.write_all(b"        match self {\n")?;
    for ttype in &types
        .iter()
        .map(|s| s.split(':').collect::<Vec<&str>>()[0].trim())
        .collect::<Vec<&str>>()
    {
        file.write_all(
            format!(
                "            {}::{}({}e) => {}e.accept(visitor),\n",
                base_name,
                ttype,
                ttype.chars().next().unwrap().to_lowercase(),
                ttype.chars().next().unwrap().to_lowercase()
            )
            .as_bytes(),
        )?;
    }
    file.write_all(b"        }\n")?;
    file.write_all(b"    }\n")?;
    file.write_all(b"}\n")?;

    // Define each type struct
    for t in &types {
        let splitted = t.split(':').collect::<Vec<&str>>();
        let ttype = splitted[0].trim();
        let args = splitted[1].trim();

        file.write_all(
            format!("#[derive(Debug)]\npub struct {}{} {{\n", ttype, base_name).as_bytes(),
        )?;
        for arg in args
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
        {
            let splitted_arg = arg.split(' ').collect::<Vec<&str>>();
            let arg_type = splitted_arg[0].trim();
            let arg_name = splitted_arg[1].trim();

            file.write_all(format!("    pub {}: {},\n", arg_name, arg_type).as_bytes())?;
        }
        file.write_all(b"}\n\n")?;
    }

    // Define the {base_name}Visitor trait
    file.write_all(format!("pub trait {}Visitor<T> {{\n", base_name).as_bytes())?;
    for ttype in &types
        .iter()
        .map(|s| s.split(':').collect::<Vec<&str>>()[0].trim())
        .collect::<Vec<&str>>()
    {
        file.write_all(
            format!(
                "    fn visit_{}_{}(&self, {}: &{}{}) -> Result<T, LoxError>;\n",
                ttype.to_lowercase(),
                base_name.to_lowercase(),
                base_name.to_lowercase(),
                ttype,
                base_name,
            )
            .as_bytes(),
        )?;
    }
    file.write_all(b"}")?;

    // Implement each <Type>{base_name}.accept() function
    for ttype in &types
        .iter()
        .map(|s| s.split(':').collect::<Vec<&str>>()[0].trim())
        .collect::<Vec<&str>>()
    {
        file.write_all(format!("\n\nimpl {}{} {{\n", ttype, base_name).as_bytes())?;
        file.write_all(
            format!(
                "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{\n",
                base_name,
            )
            .as_bytes(),
        )?;
        file.write_all(
            format!(
                "        visitor.visit_{}_{}(self)\n",
                ttype.to_lowercase(),
                base_name.to_lowercase()
            )
            .as_bytes(),
        )?;
        file.write_all(b"    }\n")?;
        file.write_all(b"}")?;
    }

    Ok(())
}
