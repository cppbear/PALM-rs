fn d0123<Mach: Machine>(m: Mach, d: vec128_storage) -> Mach::u32x4x4 {
    let d0: Mach::u64x2 = m.unpack(d);
    let incr =
        Mach::u64x2x4::from_lanes([m.vec([0, 0]), m.vec([1, 0]), m.vec([2, 0]), m.vec([3, 0])]);
    m.unpack((Mach::u64x2x4::from_lanes([d0, d0, d0, d0]) + incr).into())
}