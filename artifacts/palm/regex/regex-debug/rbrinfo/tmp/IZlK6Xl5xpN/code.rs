fn cmd_captures(args: &Args) -> Result<()> {
    let expr = args.parse_one()?;
    let prog = args.compiler().only_utf8(false).compile(&[expr])?;
    for (i, name) in prog.captures.iter().enumerate() {
        match *name {
            None => println!("{}", i),
            Some(ref name) => println!("{}:{}", i, name),
        }
    }
    Ok(())
}