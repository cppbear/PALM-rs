fn is_empty(&self) -> bool {
        match *self {
            ClassSet::Item(ClassSetItem::Empty(_)) => true,
            _ => false,
        }
    }