fn c_class(&mut self, ranges: &[hir::ClassUnicodeRange]) -> Result {
        assert!(!ranges.is_empty());
        if self.compiled.uses_bytes() {
            CompileClass {
                c: self,
                ranges: ranges,
            }.compile()
        } else {
            let ranges: Vec<(char, char)> =
                ranges.iter().map(|r| (r.start(), r.end())).collect();
            let hole = if ranges.len() == 1 && ranges[0].0 == ranges[0].1 {
                self.push_hole(InstHole::Char { c: ranges[0].0 })
            } else {
                self.push_hole(InstHole::Ranges { ranges: ranges })
            };
            Ok(Patch { hole: hole, entry: self.insts.len() - 1 })
        }
    }