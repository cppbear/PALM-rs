fn _add_char_class(
        &mut self,
        cls: &hir::ClassUnicode,
        reverse: bool,
    ) -> bool {
        use std::char;

        if self.class_exceeds_limits(cls_char_count(cls)) {
            return false;
        }
        let mut base = self.remove_complete();
        if base.is_empty() {
            base = vec![Literal::empty()];
        }
        for r in cls.iter() {
            let (s, e) = (r.start as u32, r.end as u32 + 1);
            for c in (s..e).filter_map(char::from_u32) {
                for mut lit in base.clone() {
                    let mut bytes = c.to_string().into_bytes();
                    if reverse {
                        bytes.reverse();
                    }
                    lit.extend(&bytes);
                    self.lits.push(lit);
                }
            }
        }
        true
    }