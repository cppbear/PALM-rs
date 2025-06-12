fn grow(&mut self, min_used: i32) {
        let mut new_size = self.mask + 1;
        while new_size <= min_used {
            new_size <<= 1;
        }

        self.fill = self.used;
        self.mask = new_size - 1;

        let old_map = std::mem::replace(
            self.map
                .as_mut()
                .expect("callers have to ensure map is allocated"),
            vec![GrowingHashmapMapElemChar::<ValueType>::default(); new_size as usize],
        );

        for elem in old_map {
            if elem.value != Default::default() {
                let j = self.lookup(elem.key);
                let new_elem = &mut self.map.as_mut().expect("map created above")[j];
                new_elem.key = elem.key;
                new_elem.value = elem.value;
                self.used -= 1;
                if self.used == 0 {
                    break;
                }
            }
        }

        self.used = self.fill;
    }