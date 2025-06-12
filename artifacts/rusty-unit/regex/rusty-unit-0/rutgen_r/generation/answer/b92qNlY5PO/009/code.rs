// Answer 0

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

#[test]
fn test_cmd_literals_prefixes_with_searcher() {
    struct MockArgs {
        cmd_prefixes: bool,
        flag_all_literals: bool,
        flag_searcher: bool,
        flag_lcp: bool,
        flag_lcs: bool,
    }

    impl MockArgs {
        fn parse_many(&self) -> Result<Vec<String>, String> {
            Ok(vec!["test1".into(), "test2".into()])
        }
        fn literals(&self, _: &Vec<String>, f: fn(LiteralSet, &String) -> LiteralSet) -> LiteralSet {
            let mut set = LiteralSet::new();
            for expr in ["test1", "test2"].iter() {
                set = f(set, &expr.to_string());
            }
            set
        }
    }

    struct LiteralSet;

    impl LiteralSet {
        fn new() -> Self { LiteralSet }
        fn union_prefixes(self, _e: &String) -> Self { self }
        fn unambiguous_prefixes(self) -> Self { self }
        fn literals(&self) -> Vec<String> { vec!["test1".into(), "test2".into()] }
    }

    let args = MockArgs {
        cmd_prefixes: true,
        flag_all_literals: false,
        flag_searcher: true,
        flag_lcp: false,
        flag_lcs: false,
    };

    let result = cmd_literals(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_literals_suffixes_with_searcher() {
    struct MockArgs {
        cmd_prefixes: bool,
        flag_all_literals: bool,
        flag_searcher: bool,
        flag_lcp: bool,
        flag_lcs: bool,
    }

    impl MockArgs {
        fn parse_many(&self) -> Result<Vec<String>, String> {
            Ok(vec!["test1".into(), "test2".into()])
        }
        fn literals(&self, _: &Vec<String>, f: fn(LiteralSet, &String) -> LiteralSet) -> LiteralSet {
            let mut set = LiteralSet::new();
            for expr in ["test1", "test2"].iter() {
                set = f(set, &expr.to_string());
            }
            set
        }
    }
    
    struct LiteralSet;

    impl LiteralSet {
        fn new() -> Self { LiteralSet }
        fn union_suffixes(self, _e: &String) -> Self { self }
        fn unambiguous_suffixes(self) -> Self { self }
        fn literals(&self) -> Vec<String> { vec!["test1".into(), "test2".into()] }
    }

    let args = MockArgs {
        cmd_prefixes: false,
        flag_all_literals: false,
        flag_searcher: true,
        flag_lcp: false,
        flag_lcs: false,
    };

    let result = cmd_literals(&args);
    assert!(result.is_ok());
}

