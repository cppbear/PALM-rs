// Answer 0

#[test]
fn test_build_exec_with_patterns() {
    struct Options {
        pats: Vec<String>,
        size_limit: usize,
        dfa_size_limit: usize,
    }

    struct Context {
        options: Options,
        bytes: bool,
        only_utf8: bool,
        match_type: MatchType,
    }

    struct Parsed {
        exprs: Vec<String>,
        prefixes: Prefixes,
        suffixes: Suffixes,
    }

    struct Compiler {}

    impl Compiler {
        fn new() -> Self {
            Compiler {}
        }

        fn size_limit(self, _size_limit: usize) -> Self {
            self
        }

        fn bytes(self, _bytes: bool) -> Self {
            self
        }

        fn dfa(self, _dfa: bool) -> Self {
            self
        }

        fn only_utf8(self, _only_utf8: bool) -> Self {
            self
        }

        fn reverse(self, _reverse: bool) -> Self {
            self
        }

        fn compile(self, _exprs: &Vec<String>) -> Result<Program, Error> {
            Ok(Program::new())
        }
    }

    struct Program {}

    impl Program {
        fn new() -> Self {
            Program {}
        }
    }

    struct ExecReadOnly {
        res: Vec<String>,
        nfa: Program,
        dfa: Program,
        dfa_reverse: Program,
        suffixes: LiteralSearcher,
        match_type: MatchType,
    }

    struct Exec {
        ro: Arc<ExecReadOnly>,
        cache: CachedThreadLocal,
    }

    struct LiteralSearcher;

    impl LiteralSearcher {
        fn empty() -> Self {
            LiteralSearcher {}
        }

        fn prefixes(_prefixes: Vec<String>) -> Self {
            LiteralSearcher {}
        }

        fn suffixes(_suffixes: Vec<String>) -> Self {
            LiteralSearcher {}
        }
    }

    struct MatchType;

    impl MatchType {
        fn Nothing() -> Self {
            MatchType {}
        }
    }

    struct CachedThreadLocal;

    impl CachedThreadLocal {
        fn new() -> Self {
            CachedThreadLocal {}
        }
    }

    impl Context {
        fn parse(&self) -> Result<Parsed, Error> {
            Ok(Parsed {
                exprs: vec![String::from("a"), String::from("b")],
                prefixes: Prefixes {},
                suffixes: Suffixes {},
            })
        }

        fn build(self) -> Result<Exec, Error> {
            if self.options.pats.is_empty() {
                let ro = Arc::new(ExecReadOnly {
                    res: vec![],
                    nfa: Program::new(),
                    dfa: Program::new(),
                    dfa_reverse: Program::new(),
                    suffixes: LiteralSearcher::empty(),
                    match_type: MatchType::Nothing(),
                });
                return Ok(Exec { ro: ro, cache: CachedThreadLocal::new() });
            }
            let parsed = self.parse()?;
            let mut nfa = Compiler::new()
                .size_limit(self.options.size_limit)
                .bytes(self.bytes)
                .only_utf8(self.only_utf8)
                .compile(&parsed.exprs)?;
            let mut dfa = Compiler::new()
                .size_limit(self.options.size_limit)
                .dfa(true)
                .only_utf8(self.only_utf8)
                .compile(&parsed.exprs)?;
            let mut dfa_reverse = Compiler::new()
                .size_limit(self.options.size_limit)
                .dfa(true)
                .only_utf8(self.only_utf8)
                .reverse(true)
                .compile(&parsed.exprs)?;

            let prefixes = parsed.prefixes.unambiguous_prefixes();
            let suffixes = parsed.suffixes.unambiguous_suffixes();
            nfa.prefixes = LiteralSearcher::prefixes(prefixes);
            dfa.prefixes = nfa.prefixes.clone();
            dfa.dfa_size_limit = self.options.dfa_size_limit;
            dfa_reverse.dfa_size_limit = self.options.dfa_size_limit;

            let mut ro = ExecReadOnly {
                res: self.options.pats,
                nfa: nfa,
                dfa: dfa,
                dfa_reverse: dfa_reverse,
                suffixes: LiteralSearcher::suffixes(suffixes),
                match_type: MatchType::Nothing(),
            };

            ro.match_type = ro.choose_match_type(self.match_type);

            let ro = Arc::new(ro);
            Ok(Exec { ro: ro, cache: CachedThreadLocal::new() })
        }
    }

    struct Prefixes;
    struct Suffixes;

    impl Suffixes {
        fn unambiguous_suffixes(&self) -> Vec<String> {
            vec![String::from("x"), String::from("y")]
        }
    }

    impl Prefixes {
        fn unambiguous_prefixes(&self) -> Vec<String> {
            vec![String::from("p"), String::from("q")]
        }
    }

    let context = Context {
        options: Options {
            pats: vec![String::from("pattern1"), String::from("pattern2")],
            size_limit: 1000,
            dfa_size_limit: 500,
        },
        bytes: false,
        only_utf8: false,
        match_type: MatchType::Nothing(),
    };

    let result = context.build();
    assert!(result.is_ok());
}

