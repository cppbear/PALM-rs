fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    // Matches the atomic ordering used in libcore for the Debug impl
                    self.load(Ordering::Relaxed).serialize(serializer)
                }