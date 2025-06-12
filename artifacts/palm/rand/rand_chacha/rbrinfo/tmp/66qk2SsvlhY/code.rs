pub(crate) fn round<V: ArithOps + BitOps32>(mut x: State<V>) -> State<V> {
    x.a += x.b;
    x.d = (x.d ^ x.a).rotate_each_word_right16();
    x.c += x.d;
    x.b = (x.b ^ x.c).rotate_each_word_right20();
    x.a += x.b;
    x.d = (x.d ^ x.a).rotate_each_word_right24();
    x.c += x.d;
    x.b = (x.b ^ x.c).rotate_each_word_right25();
    x
}