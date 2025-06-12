fn c_repeat_zero_or_one(&mut self, expr: &Hir, greedy: bool) -> Result {
        let split_entry = self.insts.len();
        let split = self.push_split_hole();
        let Patch { hole: hole_rep, entry: entry_rep } = self.c(expr)?;

        let split_hole = if greedy {
            self.fill_split(split, Some(entry_rep), None)
        } else {
            self.fill_split(split, None, Some(entry_rep))
        };
        let holes = vec![hole_rep, split_hole];
        Ok(Patch { hole: Hole::Many(holes), entry: split_entry })
    }