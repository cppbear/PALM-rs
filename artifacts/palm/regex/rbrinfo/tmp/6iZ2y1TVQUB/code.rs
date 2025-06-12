pub fn exec(
        prog: &'r Program,
        cache: &ProgramCache,
        matches: &'m mut [bool],
        slots: &'s mut [Slot],
        input: I,
        start: usize,
    ) -> bool {
        let mut cache = cache.borrow_mut();
        let cache = &mut cache.backtrack;
        let start = input.at(start);
        let mut b = Bounded {
            prog: prog,
            input: input,
            matches: matches,
            slots: slots,
            m: cache,
        };
        b.exec_(start)
    }