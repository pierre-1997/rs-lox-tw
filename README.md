# Rust Lox Tree-Walk Interpreter
This is the first part of the Crafting Interpreters
[book](https://craftinginterpreters.com/contents.html). It contains all the code necessary to
run (by interpreting) `lox` files (see examples in `data/`).

# TODO

- [ ] Feature: Add support for comma expressions (Chapter 6 Challenge 1)
- [ ] Feature: Handle binary operator without left member (Chapter 6 Challenge 3)
- [ ] Feature: Add support for in-line if '?:' (Chapter 6 Challenge 2)
- [ ] Change: Addition with at least member is a string = concat (Chapter 7 Challenge 2)
- [ ] Feature: Error for division by 0 (Chapter 7 Challenge 3)
- [ ] Feature: block comments. (Chapter 4 Challenge 4)
- [ ] Feature: detect and error on uninitialized variable access. (Chapter 8 Challenge 2)
- [ ] Feature: Add support of 'break' statement in loops. (Chapter 9 Challenge 3)
- [ ] Feature: Support for anonymous/lambda functions (Chapter 10 Challenge 2)
- [ ] Feature: New error/warning detection for the resolver -> Detect unused variables (Chapter 11 Challenge 3)
- [ ] Change: improve error handling
  - [ ] Feature: use Token.src_start and Token.src_end to display precise error locations.
  - [ ] Change: Custom msg string for RunTimeError (example: InvalidArgsCount)
  - [ ] Add levels (warning, error)
  - [ ] Add error code/type + maybe documentation
- [ ] Change: improve variable storage system (Chapter 11 Challenge 4)
  - [ ] Environment storing variables by unique id (index) instead of name
  - [ ] Parser responsible of declaring variables by unique id in the env
  - [ ] Interpreter can now use look up by id
- [ ] Feature: class static methods (Chapter 12 Challenge 1)
- [ ] Feature: Getter methods (Chapter 12 Challenge 2)
