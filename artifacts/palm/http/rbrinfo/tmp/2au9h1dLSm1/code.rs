fn try_append<T>(self, map: &mut HeaderMap<T>, val: T) -> Result<bool, MaxSizeReached> {
            HdrName::from_static(self, move |hdr| map.try_append2(hdr, val))
        }