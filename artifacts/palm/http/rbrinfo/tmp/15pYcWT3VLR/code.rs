fn size_hint(&self) -> (usize, Option<usize>) {
        match (&self.first, &self.next) {
            // Exactly 1
            (&Some(_), &None) => (1, Some(1)),
            // 1 + extras
            (&Some(_), Some(extras)) => {
                let (l, u) = extras.size_hint();
                (l + 1, u.map(|u| u + 1))
            }
            // Extras only
            (&None, Some(extras)) => extras.size_hint(),
            // No more
            (&None, &None) => (0, Some(0)),
        }
    }