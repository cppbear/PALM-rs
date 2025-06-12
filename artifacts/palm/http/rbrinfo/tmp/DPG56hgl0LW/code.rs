fn try_append<T>(self, map: &mut HeaderMap<T>, val: T) -> Result<bool, MaxSizeReached> {
            map.try_append2(self, val)
        }