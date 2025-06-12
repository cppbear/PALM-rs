fn c_repeat_zero_or_more(&mut self, expr: &Hir, greedy: bool) -> Result {
        let split_entry = self.insts.len();
        let split = self.push_split_hole();
        let Patch { hole: hole_rep, entry: entry_rep } = self.c(expr)?;

        self.fill(hole_rep, split_entry);
        let split_hole = if greedy {
            self.fill_split(split, Some(entry_rep), None)
        } else {
            self.fill_split(split, None, Some(entry_rep))
        };
        Ok(Patch { hole: split_hole, entry: split_entry })
    }