// Answer 0

#[test]
fn test_build_with_empty_patterns() {
    use std::sync::Arc;

    #[derive(Default)]
    struct Options {
        pats: Vec<String>,
        size_limit: usize,
        dfa_size_limit: usize,
    }

    struct Builder {
        options: Options,
        bytes: bool,
        only_utf8: bool,
        match_type: MatchType,
    }

    #[derive(Default)]
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

    #[derive(Default)]
    struct Program;

    #[derive(Default)]
    struct LiteralSearcher;

    impl LiteralSearcher {
        fn empty() -> Self {
            LiteralSearcher::default()
        }
    }

    #[derive(Default)]
    struct CachedThreadLocal;

    #[derive(Default)]
    struct MatchType;

    impl Builder {
        fn build(self) -> Result<Exec, ()> {
            if self.options.pats.is_empty() {
                let ro = Arc::new(ExecReadOnly {
                    res: vec![],
                    nfa: Program::default(),
                    dfa: Program::default(),
                    dfa_reverse: Program::default(),
                    suffixes: LiteralSearcher::empty(),
                    match_type: MatchType::default(),
                });
                return Ok(Exec { ro: ro, cache: CachedThreadLocal::default() });
            }
            // Other cases omitted for brevity...
            Err(())
        }
    }

    let builder = Builder {
        options: Options {
            pats: vec![],
            size_limit: 100,
            dfa_size_limit: 100,
        },
        bytes: false,
        only_utf8: false,
        match_type: MatchType::default(),
    };

    let result = builder.build();
    assert!(result.is_ok());
    let exec = result.unwrap();
    assert_eq!(exec.ro.res, vec![]);
}

