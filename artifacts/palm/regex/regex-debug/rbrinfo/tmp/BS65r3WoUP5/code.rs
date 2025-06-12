fn cmd_compile(args: &Args) -> Result<()> {
    let exprs = args.parse_many()?;
    let compiler =
        args.compiler()
            .bytes(args.flag_bytes)
            .only_utf8(!args.flag_bytes)
            .dfa(args.flag_dfa)
            .reverse(args.flag_dfa_reverse);
    let prog = compiler.compile(&exprs)?;
    print!("{:?}", prog);
    Ok(())
}