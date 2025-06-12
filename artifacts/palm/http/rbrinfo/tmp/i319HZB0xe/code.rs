fn try_entry<T>(self, map: &mut HeaderMap<T>) -> Result<Entry<'_, T>, TryEntryError> {
            self.as_str().try_entry(map)
        }