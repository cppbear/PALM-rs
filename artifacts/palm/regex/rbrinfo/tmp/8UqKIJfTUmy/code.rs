fn next(&mut self) -> Option<Option<&'r str>> {
        self.0
            .next()
            .as_ref()
            .map(|slot| slot.as_ref().map(|name| name.as_ref()))
    }