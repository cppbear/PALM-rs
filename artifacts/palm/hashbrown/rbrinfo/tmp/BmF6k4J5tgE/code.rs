pub fn new_in(alloc: A) -> Self {
        Self {
            map: HashMap::new_in(alloc),
        }
    }