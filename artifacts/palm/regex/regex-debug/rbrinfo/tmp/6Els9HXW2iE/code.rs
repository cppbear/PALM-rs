fn cmd_hir(args: &Args) -> Result<()> {
    use syntax::ParserBuilder;

    let mut parser = ParserBuilder::new()
        .allow_invalid_utf8(false)
        .build();
    let hir = parser.parse(&args.arg_pattern)?;
    println!("{:#?}", hir);
    Ok(())
}