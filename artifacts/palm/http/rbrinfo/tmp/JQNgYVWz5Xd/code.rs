fn try_insert<T>(
            self,
            map: &mut HeaderMap<T>,
            val: T,
        ) -> Result<Option<T>, MaxSizeReached> {
            map.try_insert2(self, val)
        }