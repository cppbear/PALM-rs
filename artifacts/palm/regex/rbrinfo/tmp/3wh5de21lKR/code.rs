fn c_class_bytes(&mut self, ranges: &[hir::ClassBytesRange]) -> Result {
        debug_assert!(!ranges.is_empty());

        let first_split_entry = self.insts.len();
        let mut holes = vec![];
        let mut prev_hole = Hole::None;
        for r in &ranges[0..ranges.len() - 1] {
            self.fill_to_next(prev_hole);
            let split = self.push_split_hole();
            let next = self.insts.len();
            self.byte_classes.set_range(r.start(), r.end());
            holes.push(self.push_hole(InstHole::Bytes {
                start: r.start(), end: r.end(),
            }));
            prev_hole = self.fill_split(split, Some(next), None);
        }
        let next = self.insts.len();
        let r = &ranges[ranges.len() - 1];
        self.byte_classes.set_range(r.start(), r.end());
        holes.push(self.push_hole(InstHole::Bytes {
            start: r.start(), end: r.end(),
        }));
        self.fill(prev_hole, next);
        Ok(Patch { hole: Hole::Many(holes), entry: first_split_entry })
    }