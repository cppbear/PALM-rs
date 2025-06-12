fn try_insert<T>(
            self,
            map: &mut HeaderMap<T>,
            val: T,
        ) -> Result<Option<T>, MaxSizeReached> {
            HdrName::from_static(self, move |hdr| map.try_insert2(hdr, val))
        }