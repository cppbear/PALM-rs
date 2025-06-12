fn main() {
    let mut args: Args = Docopt::new(USAGE)
                                .and_then(|d| d.deserialize())
                                .unwrap_or_else(|e| e.exit());
    if args.flag_dfa_reverse {
        args.flag_dfa = true;
    }
    match run(&args) {
        Ok(_) => process::exit(0),
        Err(err) => {
            let _ = writeln!(&mut io::stderr(), "{}", err);
            process::exit(1)
        }
    }
}