fn add_pos<Mach: Machine>(m: Mach, d: Mach::u32x4, i: u64) -> Mach::u32x4 {
    let d0: Mach::u64x2 = m.unpack(d.into());
    let incr = m.vec([i, 0]);
    m.unpack((d0 + incr).into())
}