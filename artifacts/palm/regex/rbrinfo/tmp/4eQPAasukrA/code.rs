fn c_repeat_range(
        &mut self,
        expr: &Hir,
        greedy: bool,
        min: u32,
        max: u32,
    ) -> Result {
        let (min, max) = (u32_to_usize(min), u32_to_usize(max));
        let patch_concat = self.c_concat(iter::repeat(expr).take(min))?;
        let initial_entry = patch_concat.entry;
        if min == max {
            return Ok(patch_concat);
        }
        // It is much simpler to compile, e.g., `a{2,5}` as:
        //
        //     aaa?a?a?
        //
        // But you end up with a sequence of instructions like this:
        //
        //     0: 'a'
        //     1: 'a',
        //     2: split(3, 4)
        //     3: 'a'
        //     4: split(5, 6)
        //     5: 'a'
        //     6: split(7, 8)
        //     7: 'a'
        //     8: MATCH
        //
        // This is *incredibly* inefficient because the splits end
        // up forming a chain, which has to be resolved everything a
        // transition is followed.
        let mut holes = vec![];
        let mut prev_hole = patch_concat.hole;
        for _ in min..max {
            self.fill_to_next(prev_hole);
            let split = self.push_split_hole();
            let Patch { hole, entry } = self.c(expr)?;
            prev_hole = hole;
            if greedy {
                holes.push(self.fill_split(split, Some(entry), None));
            } else {
                holes.push(self.fill_split(split, None, Some(entry)));
            }
        }
        holes.push(prev_hole);
        Ok(Patch { hole: Hole::Many(holes), entry: initial_entry })
    }