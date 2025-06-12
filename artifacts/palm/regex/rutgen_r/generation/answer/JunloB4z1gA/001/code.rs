// Answer 0

#[test]
fn test_class_set_item_union_span() {
    struct Span {
        start: usize,
        end: usize,
    }

    struct ClassSetItemUnion {
        span: Span,
    }

    struct ClassSetItem {
        item: ClassSetItemType,
    }

    enum ClassSetItemType {
        Union(ClassSetItemUnion),
        // other variants are omitted for brevity
    }

    fn union_span(item: &ClassSetItem) -> &Span {
        match &item.item {
            ClassSetItemType::Union(ref x) => &x.span,
        }
    }

    let span = Span { start: 0, end: 5 };
    let union_item = ClassSetItem {
        item: ClassSetItemType::Union(ClassSetItemUnion { span }),
    };

    let result = union_span(&union_item);
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 5);
}

