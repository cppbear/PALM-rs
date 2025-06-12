fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
        let kind = match rep.op.kind {
            ast::RepetitionKind::ZeroOrOne => hir::RepetitionKind::ZeroOrOne,
            ast::RepetitionKind::ZeroOrMore => hir::RepetitionKind::ZeroOrMore,
            ast::RepetitionKind::OneOrMore => hir::RepetitionKind::OneOrMore,
            ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(m)) => {
                hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(m))
            }
            ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(m)) => {
                hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(m))
            }
            ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(m,n)) => {
                hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(m, n))
            }
        };
        let greedy =
            if self.flags().swap_greed() {
                !rep.greedy
            } else {
                rep.greedy
            };
        Hir::repetition(hir::Repetition {
            kind: kind,
            greedy: greedy,
            hir: Box::new(expr),
        })
    }