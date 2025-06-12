fn next(&mut self) -> Option<usize> {
        loop {
            match self.0.next() {
                None => return None,
                Some((_, &false)) => {}
                Some((i, &true)) => return Some(i),
            }
        }
    }