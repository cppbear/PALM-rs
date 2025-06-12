fn cmd_anchors(args: &Args) -> Result<()> {
    let expr = args.parse_one()?;
    if expr.is_anchored_start() {
        println!("start");
    }
    if expr.is_anchored_end() {
        println!("end");
    }
    Ok(())
}