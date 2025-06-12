fn c_concat<'a, I>(&mut self, exprs: I) -> Result
            where I: IntoIterator<Item=&'a Hir> {
        let mut exprs = exprs.into_iter();
        let first = match exprs.next() {
            Some(expr) => expr,
            None => {
                return Ok(Patch { hole: Hole::None, entry: self.insts.len() })
            }
        };
        let Patch { mut hole, entry } = self.c(first)?;
        for e in exprs {
            let p = self.c(e)?;
            self.fill(hole, p.entry);
            hole = p.hole;
        }
        Ok(Patch { hole: hole, entry: entry })
    }