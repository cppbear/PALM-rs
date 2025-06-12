fn from_state_incr(state: u128, increment: u128) -> Self {
        let mut pcg = Lcg128Xsl64 { state, increment };
        // Move away from initial value:
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }