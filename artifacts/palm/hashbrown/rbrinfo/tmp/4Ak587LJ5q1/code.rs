fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EntryRef::Vacant(ref v) => f.debug_tuple("EntryRef").field(v).finish(),
            EntryRef::Occupied(ref o) => f.debug_tuple("EntryRef").field(o).finish(),
        }
    }