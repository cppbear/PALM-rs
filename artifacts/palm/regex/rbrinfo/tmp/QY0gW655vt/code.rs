fn c_alternate(&mut self, exprs: &[Hir]) -> Result {
        debug_assert!(
            exprs.len() >= 2, "alternates must have at least 2 exprs");

        // Initial entry point is always the first split.
        let first_split_entry = self.insts.len();

        // Save up all of the holes from each alternate. They will all get
        // patched to point to the same location.
        let mut holes = vec![];

        let mut prev_hole = Hole::None;
        for e in &exprs[0..exprs.len() - 1] {
            self.fill_to_next(prev_hole);
            let split = self.push_split_hole();
            let prev_entry = self.insts.len();
            let Patch { hole, entry } = self.c(e)?;
            if prev_entry == self.insts.len() {
                // TODO(burntsushi): It is kind of silly that we don't support
                // empty-subexpressions in alternates, but it is supremely
                // awkward to support them in the existing compiler
                // infrastructure. This entire compiler needs to be thrown out
                // anyway, so don't feel too bad.
                return Err(Error::Syntax(
                    "alternations cannot currently contain \
                     empty sub-expressions".to_string()));
            }
            holes.push(hole);
            prev_hole = self.fill_split(split, Some(entry), None);
        }
        let prev_entry = self.insts.len();
        let Patch { hole, entry } = self.c(&exprs[exprs.len() - 1])?;
        if prev_entry == self.insts.len() {
            // TODO(burntsushi): See TODO above.
            return Err(Error::Syntax(
                "alternations cannot currently contain \
                 empty sub-expressions".to_string()));
        }
        holes.push(hole);
        self.fill(prev_hole, entry);
        Ok(Patch { hole: Hole::Many(holes), entry: first_split_entry })
    }