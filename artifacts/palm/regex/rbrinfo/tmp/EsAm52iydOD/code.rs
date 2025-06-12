pub fn build(self) -> Result<Exec, Error> {
        // Special case when we have no patterns to compile.
        // This can happen when compiling a regex set.
        if self.options.pats.is_empty() {
            let ro = Arc::new(ExecReadOnly {
                res: vec![],
                nfa: Program::new(),
                dfa: Program::new(),
                dfa_reverse: Program::new(),
                suffixes: LiteralSearcher::empty(),
                match_type: MatchType::Nothing,
            });
            return Ok(Exec { ro: ro, cache: CachedThreadLocal::new() });
        }
        let parsed = self.parse()?;
        let mut nfa =
            Compiler::new()
                     .size_limit(self.options.size_limit)
                     .bytes(self.bytes || parsed.bytes)
                     .only_utf8(self.only_utf8)
                     .compile(&parsed.exprs)?;
        let mut dfa =
            Compiler::new()
                     .size_limit(self.options.size_limit)
                     .dfa(true)
                     .only_utf8(self.only_utf8)
                     .compile(&parsed.exprs)?;
        let mut dfa_reverse =
            Compiler::new()
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
            match_type: MatchType::Nothing,
        };
        ro.match_type = ro.choose_match_type(self.match_type);

        let ro = Arc::new(ro);
        Ok(Exec { ro: ro, cache: CachedThreadLocal::new() })
    }