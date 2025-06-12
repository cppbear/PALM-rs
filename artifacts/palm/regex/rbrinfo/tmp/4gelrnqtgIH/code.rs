fn clone(&self) -> Exec {
        Exec {
            ro: self.ro.clone(),
            cache: CachedThreadLocal::new(),
        }
    }