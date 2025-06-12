fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("ClassBytesRange");
        if self.start <= 0x7F {
            debug.field("start", &(self.start as char));
        } else {
            debug.field("start", &self.start);
        }
        if self.end <= 0x7F {
            debug.field("end", &(self.end as char));
        } else {
            debug.field("end", &self.end);
        }
        debug.finish()
    }