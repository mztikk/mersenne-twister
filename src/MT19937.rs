// More information, including original source can be found at the following
// Link: http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/MT2002/emt19937ar.html

/*
   A C-program for MT19937, with initialization improved 2002/1/26.
   Coded by Takuji Nishimura and Makoto Matsumoto.

   Before using, initialize the state by using init_genrand(seed)
   or init_by_array(init_key, key_length).

   Copyright (C) 1997 - 2002, Makoto Matsumoto and Takuji Nishimura,
   All rights reserved.

   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

     1. Redistributions of source code must retain the above copyright
        notice, this list of conditions and the following disclaimer.

     2. Redistributions in binary form must reproduce the above copyright
        notice, this list of conditions and the following disclaimer in the
        documentation and/or other materials provided with the distribution.

     3. The names of its contributors may not be used to endorse or promote
        products derived from this software without specific prior written
        permission.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
   CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


   Any feedback is very welcome.
   http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/emt.html
   email: m-mat @ math.sci.hiroshima-u.ac.jp (remove space)
*/

pub const DEFAULT_SEED: u32 = 5489;
pub const DEFAULT_SEED_PS2: u32 = 4537;

const N: usize = 624;
const M: usize = 397;

const MATRIX_A: u32 = 0x9908b0df;
const UPPER_MASK: u32 = 0x80000000;
const LOWER_MASK: u32 = 0x7fffffff;

#[inline]
const fn temper(y: u32) -> u32 {
    let mut y = y;
    y ^= y >> 11;
    y ^= (y << 7) & 0x9d2c5680;
    y ^= (y << 15) & 0xefc60000;
    y ^= y >> 18;

    y
}

#[inline]
fn twist(state: &mut [u32; 624]) {
    for i in 0..N {
        let x = (state[i] & UPPER_MASK) + (state[(i + 1) % N] & LOWER_MASK);
        let mut x_a = x >> 1;

        if x % 2 != 0 {
            x_a ^= MATRIX_A;
        }

        state[i] = state[(i + M) % N] ^ x_a;
    }
}

const LOOKAHEAD_SIZE: usize = 10;
const MAX_LOOKAHEAD_SIZE: usize = N * LOOKAHEAD_SIZE;

pub struct MTState {
    pub mti: usize,
    pub mt: u32,
    pub value: u32,
}

impl MTState {
    pub const fn new(mti: usize, mt: u32, value: u32) -> Self {
        MTState { mti, mt, value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MT19937 {
    states: [[u32; N]; LOOKAHEAD_SIZE],
    index: usize,
}

impl MT19937 {
    pub fn new() -> Self {
        MT19937::new_with_seed(DEFAULT_SEED)
    }

    pub fn new_with_seed(seed: u32) -> Self {
        let mut mt = MT19937 {
            states: [[0; N]; LOOKAHEAD_SIZE],
            index: 0,
        };
        mt.sgenrand(seed);
        mt.init_states();
        mt
    }

    pub fn sgenrand(&mut self, seed: u32) {
        let state = &mut self.states[0];
        state[0] = seed;

        self.index = 1;
        while self.index < N {
            state[self.index] = 1812433253_u32
                .wrapping_mul(state[self.index - 1] ^ (state[self.index - 1] >> 30))
                + self.index as u32;
            state[self.index] &= 0xffffffff;

            self.index += 1;
        }
    }

    pub fn genrand(&mut self) -> u32 {
        if self.index >= N {
            self.init_states();
        }

        let y = self.states[0][self.index];
        self.index += 1;

        temper(y)
    }

    pub fn load_state(&mut self, state: &[u32; 624], index: usize) {
        self.states[0] = *state;
        self.build_lookahead_states();
        self.index = index;
    }

    fn build_lookahead_states(&mut self) {
        for i in 1..LOOKAHEAD_SIZE {
            let previous_state = &self.states[i - 1];
            self.states[i] = *previous_state;
            let state = &mut self.states[i];
            twist(state);
        }
    }

    fn init_states(&mut self) {
        twist(&mut self.states[0]);
        self.build_lookahead_states();

        self.index = 0;
    }

    const fn get_max_lookahead_index(&self) -> usize {
        MAX_LOOKAHEAD_SIZE - self.index - 1
    }

    pub const fn peek_specific(&self, state_index: usize, index: usize) -> Option<u32> {
        match self.peek_specific_state(state_index, index) {
            Some(state) => Some(state.value),
            None => None,
        }
    }

    pub const fn peek(&self, offset: usize) -> Option<u32> {
        match self.peek_state(offset) {
            Some(state) => Some(state.value),
            None => None,
        }
    }

    pub const fn peek_specific_state(&self, state_index: usize, index: usize) -> Option<MTState> {
        if state_index > self.get_max_lookahead_index() {
            return None;
        }

        if index >= N {
            return None;
        }

        let mt = self.states[state_index][index];
        let value = temper(mt);

        Some(MTState::new(index, mt, value))
    }

    pub const fn peek_state(&self, offset: usize) -> Option<MTState> {
        let offset_index = self.index + offset;
        let state_index = offset_index / N;
        let index = offset_index % N;

        self.peek_specific_state(state_index, index)
    }
}

impl Default for MT19937 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::MT19937::{MAX_LOOKAHEAD_SIZE, N};

    #[test]
    fn test_default() {
        let mut mt = super::MT19937::default();

        assert_eq!(mt.genrand(), 3499211612);
        assert_eq!(mt.genrand(), 581869302);
        assert_eq!(mt.genrand(), 3890346734);
        assert_eq!(mt.genrand(), 3586334585);
        assert_eq!(mt.genrand(), 545404204);
    }

    #[test]
    fn test_default_seed() {
        let mut mt = super::MT19937::new_with_seed(super::DEFAULT_SEED);

        assert_eq!(mt.genrand(), 3499211612);
        assert_eq!(mt.genrand(), 581869302);
        assert_eq!(mt.genrand(), 3890346734);
        assert_eq!(mt.genrand(), 3586334585);
        assert_eq!(mt.genrand(), 545404204);
    }

    #[test]
    fn test_default_seed_ps2() {
        let mut mt = super::MT19937::new_with_seed(super::DEFAULT_SEED_PS2);

        assert_eq!(mt.genrand(), 1288459236);
        assert_eq!(mt.genrand(), 2139177191);
        assert_eq!(mt.genrand(), 74803024);
        assert_eq!(mt.genrand(), 3048110697);
        assert_eq!(mt.genrand(), 1213569425);
    }

    #[test]
    fn test_default_peek() {
        let mt = super::MT19937::default();

        assert_eq!(mt.peek(0), Some(3499211612));
        assert_eq!(mt.peek(1), Some(581869302));
        assert_eq!(mt.peek(2), Some(3890346734));
        assert_eq!(mt.peek(3), Some(3586334585));
        assert_eq!(mt.peek(4), Some(545404204));
    }

    #[test]
    fn test_ps2_peek_next_state() {
        let mt = super::MT19937::new_with_seed(super::DEFAULT_SEED_PS2);

        assert_eq!(mt.peek(624), Some(2370195708));
        assert_eq!(mt.peek(624 + 1), Some(1272340656));
        assert_eq!(mt.peek(624 + 2), Some(2451137865));
        assert_eq!(mt.peek(624 + 3), Some(1725072322));
        assert_eq!(mt.peek(624 + 4), Some(781178099));
    }

    #[test]
    fn test_ps2_peek_specific_state() {
        let mt = super::MT19937::new_with_seed(super::DEFAULT_SEED_PS2);

        assert_eq!(mt.peek_specific(1, 0), Some(2370195708));
        assert_eq!(mt.peek_specific(1, 1), Some(1272340656));
        assert_eq!(mt.peek_specific(1, 2), Some(2451137865));
        assert_eq!(mt.peek_specific(1, 3), Some(1725072322));
        assert_eq!(mt.peek_specific(1, 4), Some(781178099));

        assert_eq!(mt.peek_specific(2, 0), Some(3776761649));
        assert_eq!(mt.peek_specific(2, 1), Some(4155410904));
        assert_eq!(mt.peek_specific(2, 2), Some(1258679099));
        assert_eq!(mt.peek_specific(2, 3), Some(2085559062));
        assert_eq!(mt.peek_specific(2, 4), Some(2548599404));
    }

    #[test]
    fn test_non_existent_peek() {
        let mt = super::MT19937::default();

        assert_eq!(mt.peek_specific(MAX_LOOKAHEAD_SIZE + 1, N + 1), None);
        assert_eq!(mt.peek_specific(MAX_LOOKAHEAD_SIZE + 1, N), None);
        assert_eq!(mt.peek_specific(MAX_LOOKAHEAD_SIZE + 1, N - 1), None);
        assert_eq!(mt.peek_specific(MAX_LOOKAHEAD_SIZE, N), None);
        assert_eq!(mt.peek_specific(MAX_LOOKAHEAD_SIZE - 1, N), None);
        assert_eq!(mt.peek(N * MAX_LOOKAHEAD_SIZE), None);
    }
}
