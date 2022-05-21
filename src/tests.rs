/*

    #[test]
    fn test_error_variable_already_exists() {

    let code = "fun bad() {
      var a = \"first\";
      var a = \"second\";
    }";


    assert_eq!(lox.run(code), Err(LoxResult::Resolver {
                    token: name.clone(),
                    error_type: ResolverErrorType::VariableAlreadyExists,
                }))
    }
*/
