pub fn searcher(&self) -> ExecNoSync {
        let create = || Box::new(RefCell::new(ProgramCacheInner::new(&self.ro)));
        ExecNoSync {
            ro: &self.ro, // a clone is too expensive here! (and not needed)
            cache: self.cache.get_or(create),
        }
    }