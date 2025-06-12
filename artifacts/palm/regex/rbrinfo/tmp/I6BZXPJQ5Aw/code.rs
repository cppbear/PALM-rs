fn c_empty_look(&mut self, look: EmptyLook) -> Result {
        let hole = self.push_hole(InstHole::EmptyLook { look: look });
        Ok(Patch { hole: hole, entry: self.insts.len() - 1 })
    }