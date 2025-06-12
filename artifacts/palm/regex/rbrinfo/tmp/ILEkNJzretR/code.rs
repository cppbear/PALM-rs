fn c_bytes(&mut self, bytes: &[u8]) -> Result {
        debug_assert!(!bytes.is_empty());
        let mut bytes: Box<Iterator<Item=&u8>> =
            if self.compiled.is_reverse {
                Box::new(bytes.iter().rev())
            } else {
                Box::new(bytes.iter())
            };
        let first = *bytes.next().expect("non-empty literal");
        let Patch { mut hole, entry } = self.c_byte(first)?;
        for &b in bytes {
            let p = self.c_byte(b)?;
            self.fill(hole, p.entry);
            hole = p.hole;
        }
        Ok(Patch { hole: hole, entry: entry })
    }