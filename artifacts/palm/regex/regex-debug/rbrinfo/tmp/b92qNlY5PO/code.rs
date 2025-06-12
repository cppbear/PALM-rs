fn cmd_literals(args: &Args) -> Result<()> {
    let exprs = args.parse_many()?;
    let mut lits =
        if args.cmd_prefixes {
            args.literals(&exprs, |lits, e| lits.union_prefixes(e))
        } else {
            args.literals(&exprs, |lits, e| lits.union_suffixes(e))
        };
    if !args.flag_all_literals {
        if args.cmd_prefixes {
            lits = lits.unambiguous_prefixes();
        } else {
            lits = lits.unambiguous_suffixes();
        }
    }
    if args.flag_searcher {
        if args.cmd_prefixes {
            println!("{:?}", LiteralSearcher::prefixes(lits))
        } else {
            println!("{:?}", LiteralSearcher::suffixes(lits))
        }
    } else if args.flag_lcp {
        println!("{}", escape_unicode(lits.longest_common_prefix()));
    } else if args.flag_lcs {
        println!("{}", escape_unicode(lits.longest_common_suffix()));
    } else {
        for lit in lits.literals() {
            println!("{:?}", lit);
        }
    }
    Ok(())
}