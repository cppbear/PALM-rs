fn check_size(&self) -> result::Result<(), Error> {
        use std::mem::size_of;

        if self.insts.len() * size_of::<Inst>() > self.size_limit {
            Err(Error::CompiledTooBig(self.size_limit))
        } else {
            Ok(())
        }
    }