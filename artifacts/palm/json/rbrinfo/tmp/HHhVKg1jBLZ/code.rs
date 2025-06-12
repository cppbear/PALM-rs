pub fn with_capacity(capacity: usize) -> Self {
        Map {
            #[cfg(not(feature = "preserve_order"))]
            map: {
                // does not support with_capacity
                let _ = capacity;
                BTreeMap::new()
            },
            #[cfg(feature = "preserve_order")]
            map: IndexMap::with_capacity(capacity),
        }
    }