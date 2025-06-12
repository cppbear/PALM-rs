fn c_repeat(&mut self, rep: &hir::Repetition) -> Result {
        use syntax::hir::RepetitionKind::*;
        match rep.kind {
            ZeroOrOne => self.c_repeat_zero_or_one(&rep.hir, rep.greedy),
            ZeroOrMore => self.c_repeat_zero_or_more(&rep.hir, rep.greedy),
            OneOrMore => self.c_repeat_one_or_more(&rep.hir, rep.greedy),
            Range(hir::RepetitionRange::Exactly(min_max)) => {
                self.c_repeat_range(&rep.hir, rep.greedy, min_max, min_max)
            }
            Range(hir::RepetitionRange::AtLeast(min)) => {
                self.c_repeat_range_min_or_more(&rep.hir, rep.greedy, min)
            }
            Range(hir::RepetitionRange::Bounded(min, max)) => {
                self.c_repeat_range(&rep.hir, rep.greedy, min, max)
            }
        }
    }