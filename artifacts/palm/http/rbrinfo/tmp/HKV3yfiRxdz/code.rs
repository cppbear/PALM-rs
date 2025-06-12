pub fn get_all<K>(&self, key: K) -> GetAll<'_, T>
    where
        K: AsHeaderName,
    {
        GetAll {
            map: self,
            index: key.find(self).map(|(_, i)| i),
        }
    }