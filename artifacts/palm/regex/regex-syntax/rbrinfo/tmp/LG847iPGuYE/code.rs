fn visit_post(&mut self, hir: &Hir) -> fmt::Result {
        match *hir.kind() {
            // Handled during visit_pre
            HirKind::Empty
            | HirKind::Literal(_)
            | HirKind::Class(_)
            | HirKind::Anchor(_)
            | HirKind::WordBoundary(_)
            | HirKind::Concat(_)
            | HirKind::Alternation(_) => {}
            HirKind::Repetition(ref x) => {
                match x.kind {
                    hir::RepetitionKind::ZeroOrOne => {
                        self.wtr.write_str("?")?;
                    }
                    hir::RepetitionKind::ZeroOrMore => {
                        self.wtr.write_str("*")?;
                    }
                    hir::RepetitionKind::OneOrMore => {
                        self.wtr.write_str("+")?;
                    }
                    hir::RepetitionKind::Range(ref x) => {
                        match *x {
                            hir::RepetitionRange::Exactly(m) => {
                                write!(self.wtr, "{{{}}}", m)?;
                            }
                            hir::RepetitionRange::AtLeast(m) => {
                                write!(self.wtr, "{{{},}}", m)?;
                            }
                            hir::RepetitionRange::Bounded(m, n) => {
                                write!(self.wtr, "{{{},{}}}", m, n)?;
                            }
                        }
                    }
                }
                if !x.greedy {
                    self.wtr.write_str("?")?;
                }
            }
            HirKind::Group(_) => {
                self.wtr.write_str(")")?;
            }
        }
        Ok(())
    }