pub fn can_exec(insts: &Program) -> bool {
    use prog::Inst::*;
    // If for some reason we manage to allocate a regex program with more
    // than i32::MAX instructions, then we can't execute the DFA because we
    // use 32 bit instruction pointer deltas for memory savings.
    // If i32::MAX is the largest positive delta,
    // then -i32::MAX == i32::MIN + 1 is the largest negative delta,
    // and we are OK to use 32 bits.
    if insts.dfa_size_limit == 0 || insts.len() > ::std::i32::MAX as usize {
        return false;
    }
    for inst in insts {
        match *inst {
            Char(_) | Ranges(_) => return false,
            EmptyLook(_) | Match(_) | Save(_) | Split(_) | Bytes(_) => {}
        }
    }
    true
}