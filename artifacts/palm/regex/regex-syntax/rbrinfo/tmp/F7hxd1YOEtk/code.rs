fn case_fold_simple(&self, ranges: &mut Vec<ClassUnicodeRange>) {
        if !unicode::contains_simple_case_mapping(self.start, self.end) {
            return;
        }
        let start = self.start as u32;
        let end = (self.end as u32).saturating_add(1);
        let mut next_simple_cp = None;
        for cp in (start..end).filter_map(char::from_u32) {
            if next_simple_cp.map_or(false, |next| cp < next) {
                continue;
            }
            let it = match unicode::simple_fold(cp) {
                Ok(it) => it,
                Err(next) => {
                    next_simple_cp = next;
                    continue;
                }
            };
            for cp_folded in it {
                ranges.push(ClassUnicodeRange::new(cp_folded, cp_folded));
            }
        }
    }