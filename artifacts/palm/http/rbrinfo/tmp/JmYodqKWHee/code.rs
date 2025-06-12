fn try_entry<T>(self, map: &mut HeaderMap<T>) -> Result<Entry<'_, T>, MaxSizeReached> {
            map.try_entry2(self)
        }