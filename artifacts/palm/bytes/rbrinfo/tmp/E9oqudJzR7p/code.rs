fn original_capacity_from_repr(repr: usize) -> usize {
    if repr == 0 {
        return 0;
    }

    1 << (repr + (MIN_ORIGINAL_CAPACITY_WIDTH - 1))
}