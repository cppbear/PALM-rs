fn index(&self, index: K) -> &T {
        match self.get2(&index) {
            Some(val) => val,
            None => panic!("no entry found for key {:?}", index.as_str()),
        }
    }