fn c_repeat_range_min_or_more(
        &mut self,
        expr: &Hir,
        greedy: bool,
        min: u32,
    ) -> Result {
        let min = u32_to_usize(min);
        let patch_concat = self.c_concat(iter::repeat(expr).take(min))?;
        let patch_rep = self.c_repeat_zero_or_more(expr, greedy)?;
        self.fill(patch_concat.hole, patch_rep.entry);
        Ok(Patch { hole: patch_rep.hole, entry: patch_concat.entry })
    }