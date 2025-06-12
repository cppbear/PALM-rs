fn lookup(&self, key: u32) -> usize {
        let hash = key;
        let mut i = hash as usize & self.mask as usize;

        let map = self
            .map
            .as_ref()
            .expect("callers have to ensure map is allocated");

        if map[i].value == Default::default() || map[i].key == key {
            return i;
        }

        let mut perturb = key;
        loop {
            i = (i * 5 + perturb as usize + 1) & self.mask as usize;

            if map[i].value == Default::default() || map[i].key == key {
                return i;
            }

            perturb >>= 5;
        }
    }