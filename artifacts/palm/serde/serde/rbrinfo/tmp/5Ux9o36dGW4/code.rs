fn next_pair(&mut self) -> Option<(First<I::Item>, Second<I::Item>)> {
        match self.iter.next() {
            Some(kv) => {
                self.count += 1;
                Some(private::Pair::split(kv))
            }
            None => None,
        }
    }