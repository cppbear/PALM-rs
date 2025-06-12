fn c_literal(&mut self, chars: &[char]) -> Result {
        debug_assert!(!chars.is_empty());
        let mut chars: Box<Iterator<Item=&char>> =
            if self.compiled.is_reverse {
                Box::new(chars.iter().rev())
            } else {
                Box::new(chars.iter())
            };
        let first = *chars.next().expect("non-empty literal");
        let Patch { mut hole, entry } = self.c_char(first)?;
        for &c in chars {
            let p = self.c_char(c)?;
            self.fill(hole, p.entry);
            hole = p.hole;
        }
        Ok(Patch { hole: hole, entry: entry })
    }