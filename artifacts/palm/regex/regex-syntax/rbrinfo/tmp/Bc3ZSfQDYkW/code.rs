pub fn union(ast: ClassSetUnion) -> ClassSet {
        ClassSet::Item(ClassSetItem::Union(ast))
    }