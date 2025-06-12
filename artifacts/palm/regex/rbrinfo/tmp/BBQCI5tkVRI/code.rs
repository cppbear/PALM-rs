pub fn new(_prog: &Program) -> Self {
        Cache {
            clist: Threads::new(),
            nlist: Threads::new(),
            stack: vec![],
        }
    }