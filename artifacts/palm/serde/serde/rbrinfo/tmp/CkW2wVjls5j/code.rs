pub fn new(name: &'static str, expecting: &'static str) -> Self {
            TaggedContentVisitor {
                tag_name: name,
                expecting,
                value: PhantomData,
            }
        }