pub(crate) fn diagonalize<V: LaneWords4>(mut x: State<V>) -> State<V> {
    x.b = x.b.shuffle_lane_words3012();
    x.c = x.c.shuffle_lane_words2301();
    x.d = x.d.shuffle_lane_words1230();
    x
}