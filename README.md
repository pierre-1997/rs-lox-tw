# TODO

- [ ] Feature: block comments.
- [ ] Future feature: use Token.src_start and Token.src_end to display precise error locations.
- [ ] Feature: detect and error on uninitialized variable access.
- [ ] Feature: Add support of 'break' statement in loops.
- [ ] Change: Custom msg string for RunTimeError (example: InvalidArgsCount)
- [ ] Feature: Support for anonymous/lambda functions
- [ ] Feature: New error/warning detection for the resolver -> Detect unused variables
- [ ] Change: improve error handling
  - [ ] Add levels (warning, error)
  - [ ] Add error code/type + maybe documentation
- [ ] Change: improve variable storage system
  - [ ] Environment storing variables by unique id (index) instead of name
  - [ ] Parser responsible of declaring variables by unique id in the env
  - [ ] Interpreter can now use look up by id
