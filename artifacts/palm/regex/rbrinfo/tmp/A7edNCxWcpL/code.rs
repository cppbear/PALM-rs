pub fn exec(
        prog: &'r Program,
        cache: &ProgramCache,
        matches: &mut [bool],
        slots: &mut [Slot],
        quit_after_match: bool,
        input: I,
        start: usize,
    ) -> bool {
        let mut cache = cache.borrow_mut();
        let cache = &mut cache.pikevm;
        cache.clist.resize(prog.len(), prog.captures.len());
        cache.nlist.resize(prog.len(), prog.captures.len());
        let at = input.at(start);
        Fsm {
            prog: prog,
            stack: &mut cache.stack,
            input: input,
        }.exec_(
            &mut cache.clist,
            &mut cache.nlist,
            matches,
            slots,
            quit_after_match,
            at,
        )
    }