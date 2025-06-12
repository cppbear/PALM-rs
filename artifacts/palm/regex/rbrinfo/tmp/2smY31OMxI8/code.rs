fn c_dotstar(&mut self) -> Result {
        Ok(if !self.compiled.only_utf8() {
            self.c(&Hir::repetition(hir::Repetition {
                kind: hir::RepetitionKind::ZeroOrMore,
                greedy: false,
                hir: Box::new(Hir::any(true)),
            }))?
        } else {
            self.c(&Hir::repetition(hir::Repetition {
                kind: hir::RepetitionKind::ZeroOrMore,
                greedy: false,
                hir: Box::new(Hir::any(false)),
            }))?
        })
    }