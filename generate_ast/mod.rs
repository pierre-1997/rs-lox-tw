use std::fs::File;
use std::io::prelude::*;

pub fn generate_ast(output_dir: &str) -> std::io::Result<()> {
    define_ast(
        output_dir,
        "expr",
        vec![
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Unary    : Token operator, Box<Expr> right".to_string(),
            "Grouping : Box<Expr> expression".to_string(),
            "Literal  : Option<Object> value".to_string(),
        ],
    )
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(output_dir.to_owned() + "/" + base_name + ".rs")?;

    // Imports
    file.write_all(b"use crate::errors::ExprError;\n")?;
    file.write_all(b"use crate::token::{Object, Token};\n\n")?;

    // Define Expr enum
    file.write_all(b"pub enum Expr {\n")?;
    for ttype in &types
        .iter()
        .map(|s| s.split(':').collect::<Vec<&str>>()[0].trim())
        .collect::<Vec<&str>>()
    {
        file.write_all(format!("    {}({}Expr),\n", ttype, ttype).as_bytes())?;
    }
    file.write_all(b"}\n\n")?;

    file.write_all(b"impl Expr {\n")?;
    file.write_all(
        b"    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ExprError> {\n",
    )?;
    file.write_all(b"        match self {\n")?;
    for ttype in &types
        .iter()
        .map(|s| s.split(':').collect::<Vec<&str>>()[0].trim())
        .collect::<Vec<&str>>()
    {
        file.write_all(
            format!(
                "            Expr::{}({}e) => {}e.accept(visitor),\n",
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

        file.write_all(format!("pub struct {}Expr {{\n", ttype).as_bytes())?;
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

    // Define the ExprVisitor trait
    file.write_all(b"pub trait ExprVisitor<T> {\n")?;
    for ttype in &types
        .iter()
        .map(|s| s.split(':').collect::<Vec<&str>>()[0].trim())
        .collect::<Vec<&str>>()
    {
        file.write_all(
            format!(
                "    fn visit_{}_expr(&self, expr: &{}Expr) -> Result<T, ExprError>;\n",
                ttype.to_lowercase(),
                ttype
            )
            .as_bytes(),
        )?;
    }
    file.write_all(b"}")?;

    // Implement each <Type>Expr.accept() function
    for ttype in &types
        .iter()
        .map(|s| s.split(':').collect::<Vec<&str>>()[0].trim())
        .collect::<Vec<&str>>()
    {
        file.write_all(format!("\n\nimpl {}Expr {{\n", ttype).as_bytes())?;
        file.write_all(
            b"    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ExprError> {\n",
        )?;
        file.write_all(
            format!(
                "        visitor.visit_{}_expr(self)\n",
                ttype.to_lowercase()
            )
            .as_bytes(),
        )?;
        file.write_all(b"    }\n")?;
        file.write_all(b"}")?;
    }

    Ok(())
}
