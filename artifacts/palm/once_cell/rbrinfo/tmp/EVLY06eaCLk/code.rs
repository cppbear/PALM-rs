pub(crate) fn wait(&self) {
        initialize_or_wait(&self.queue, None);
    }