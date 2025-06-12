fn next_back(&mut self) -> Option<usize> {
        loop {
            match self.0.next_back() {
                None => return None,
                Some((_, false)) => {}
                Some((i, true)) => return Some(i),
            }
        }
    }