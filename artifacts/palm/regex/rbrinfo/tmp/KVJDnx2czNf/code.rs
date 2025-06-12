fn c_capture(&mut self, first_slot: usize, expr: &Hir) -> Result {
        if self.num_exprs > 1 || self.compiled.is_dfa {
            // Don't ever compile Save instructions for regex sets because
            // they are never used. They are also never used in DFA programs
            // because DFAs can't handle captures.
            self.c(expr)
        } else {
            let entry = self.insts.len();
            let hole = self.push_hole(InstHole::Save { slot: first_slot });
            let patch = self.c(expr)?;
            self.fill(hole, patch.entry);
            self.fill_to_next(patch.hole);
            let hole = self.push_hole(InstHole::Save { slot: first_slot + 1 });
            Ok(Patch { hole: hole, entry: entry })
        }
    }