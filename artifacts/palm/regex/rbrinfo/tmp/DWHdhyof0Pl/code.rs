fn c_repeat_one_or_more(&mut self, expr: &Hir, greedy: bool) -> Result {
        let Patch { hole: hole_rep, entry: entry_rep } = self.c(expr)?;
        self.fill_to_next(hole_rep);
        let split = self.push_split_hole();

        let split_hole = if greedy {
            self.fill_split(split, Some(entry_rep), None)
        } else {
            self.fill_split(split, None, Some(entry_rep))
        };
        Ok(Patch { hole: split_hole, entry: entry_rep })
    }