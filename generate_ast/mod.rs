use std::fs::File;
use std::io::prelude::*;

pub fn generate_ast(output_dir: &str) -> std::io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        vec![
            "Assign   : Token name, Box<Expr> value".to_string(),
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Call     : Box<Expr> callee, Token paren, Vec<Expr> arguments".to_string(),
            "Get      : Box<Expr> object, Token name".to_string(),
            "Logical  : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Set      : Box<Expr> object, Token name, Box<Expr> value".to_string(),
            "This     : Token keyword".to_string(),
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
            "Class      : Token name, Vec<Stmt> methods".to_string(),
            "Expression : Expr expression".to_string(),
            "Function   : Token name, Vec<Token> params, Vec<Stmt> body".to_string(),
            "If         : Expr condition, Box<Stmt> then_branch, Box<Option<Stmt>> else_branch"
                .to_string(),
            "Print      : Expr expression".to_string(),
            "Return     : Token keyword, Option<Expr> value".to_string(),
            "Var        : Token name, Option<Expr> initializer".to_string(),
            "While      : Expr condition, Box<Stmt> body".to_string(),
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(output_dir.to_owned() + "/" + &base_name.to_lowercase() + ".rs")?;

    // Imports
    if base_name == "Stmt" {
        file.write_all(b"use crate::expr::Expr;\n")?;
        file.write_all(b"use crate::token::Token;\n")?;
    } else if base_name == "Expr" {
        file.write_all(b"use crate::object::Object;\n")?;
        file.write_all(b"use crate::token::Token;\n")?;
    }
    file.write_all(b"use crate::errors::LoxResult;\n")?;
    // Additional '\n' after imports
    file.write_all(b"\n")?;

    // Define Expr enum
    file.write_all(format!("#[derive(Debug, Clone)]\npub enum {} {{\n", base_name).as_bytes())?;
    // Each type will be an enum's variant
    for t in &types {
        // Get the type's name and its list of arguments separatelty
        let splitted = t.split(':').collect::<Vec<&str>>();
        let ttype = splitted[0].trim();
        let args = splitted[1].trim();

        // Declare the enum's variant using the type name
        file.write_all(format!("    {} {{\n", ttype).as_bytes())?;
        // For each argument
        for arg in args
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
        {
            // Get the argument's name and type separatelty
            let splitted_arg = arg.split(' ').collect::<Vec<&str>>();
            let arg_type = splitted_arg[0].trim();
            let arg_name = splitted_arg[1].trim();

            // Write the variant's members as the type's arguments
            file.write_all(format!("        {}: {},\n", arg_name, arg_type).as_bytes())?;
        }
        // Close the variant declaration
        file.write_all(b"    },\n")?;
    }
    // Close the enum declaration
    file.write_all(b"}\n\n")?;

    // Now implement the accept() function for the enum itself
    file.write_all(format!("impl {} {{\n", base_name).as_bytes())?;
    file.write_all(
        format!(
            "    pub fn accept<T>(&self, visitor: &mut dyn {}Visitor<T>) -> Result<T, LoxResult> {{\n",
            base_name
        )
        .as_bytes(),
    )?;
    file.write_all(b"        match self {\n")?;
    // For each type
    for t in &types {
        // Get the type's name and its list of arguments separatelty
        let splitted = t.split(':').collect::<Vec<&str>>();
        let ttype = splitted[0].trim();
        let args = splitted[1].trim();

        // For each argument
        let arg_names = args
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
            .iter()
            .map(|arg| arg.split(' ').collect::<Vec<&str>>()[1])
            .collect::<Vec<&str>>();

        // Get the argument's name and type separatelty
        file.write_all(
            format!(
                "            {}::{} {{ {} }} => visitor.visit_{}_{}({}),\n",
                base_name,
                ttype,
                arg_names.join(", "),
                ttype.to_lowercase(),
                base_name.to_lowercase(),
                arg_names.join(", "),
            )
            .as_bytes(),
        )?;
    }
    // Closing the match
    file.write_all(b"        }\n")?;
    // Closing the accept() function
    file.write_all(b"    }\n")?;
    // Closing the type impl
    file.write_all(b"}\n\n")?;

    // Define the {base_name}Visitor trait
    file.write_all(format!("pub trait {}Visitor<T> {{\n", base_name).as_bytes())?;
    for t in &types {
        // Get the type's name and its list of arguments separatelty
        let splitted = t.split(':').collect::<Vec<&str>>();
        let ttype = splitted[0].trim();
        let args = splitted[1].trim();

        // Declare the enum's variant using the type name
        file.write_all(
            format!(
                "    fn visit_{}_{}(&mut self",
                ttype.to_lowercase(),
                base_name.to_lowercase()
            )
            .as_bytes(),
        )?;
        // For each argument
        for arg in args
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
        {
            // Get the argument's name and type separatelty
            let splitted_arg = arg.split(' ').collect::<Vec<&str>>();
            let arg_type = splitted_arg[0].trim();
            let arg_name = splitted_arg[1].trim();

            if arg_type.contains("Box") {
                file.write_all(
                    format!(
                        ", {}: &{}",
                        arg_name,
                        arg_type.replace("Box<", "").strip_suffix('>').unwrap(),
                    )
                    .as_bytes(),
                )?;
            } else if arg_type.contains("Vec") {
                file.write_all(
                    format!(
                        ", {}: {}",
                        arg_name,
                        arg_type.replace("Vec<", "&[").replace('>', "]"),
                    )
                    .as_bytes(),
                )?;
            } else {
                file.write_all(format!(", {}: &{}", arg_name, arg_type,).as_bytes())?;
            }
        }
        file.write_all(b") -> Result<T, LoxResult>;\n")?;
    }
    // Close the visitor trait declaration
    file.write_all(b"}\n\n")?;

    Ok(())
}
