fn locations(&self) -> Locations {
        Locations(vec![None; self.slots_len()])
    }