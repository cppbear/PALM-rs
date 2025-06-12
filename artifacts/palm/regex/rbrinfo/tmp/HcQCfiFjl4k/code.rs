fn next(&mut self) -> Option<Option<Match<'t>>> {
        self.it.next()
            .map(|cap| cap.map(|(s, e)| Match::new(self.caps.text, s, e)))
    }