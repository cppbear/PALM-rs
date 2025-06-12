fn new(ro: &ExecReadOnly) -> Self {
        ProgramCacheInner {
            pikevm: pikevm::Cache::new(&ro.nfa),
            backtrack: backtrack::Cache::new(&ro.nfa),
            dfa: dfa::Cache::new(&ro.dfa),
            dfa_reverse: dfa::Cache::new(&ro.dfa_reverse),
        }
    }