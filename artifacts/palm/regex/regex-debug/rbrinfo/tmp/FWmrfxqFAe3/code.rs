fn cmd_ast(args: &Args) -> Result<()> {
    use syntax::ast::parse::Parser;

    let mut parser = Parser::new();
    let ast = parser.parse(&args.arg_pattern)?;
    println!("{:#?}", ast);
    Ok(())
}