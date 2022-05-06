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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MT19937 {
    state: [u32; N],
    index: usize,
}

impl MT19937 {
    pub fn new() -> Self {
        MT19937::new_with_seed(DEFAULT_SEED)
    }

    pub fn new_with_seed(seed: u32) -> Self {
        let mut mt = MT19937 {
            state: [0; 624],
            index: 0,
        };
        mt.sgenrand(seed);
        mt
    }

    pub fn sgenrand(&mut self, seed: u32) {
        self.state[0] = seed;

        self.index = 1;
        while self.index < N {
            self.state[self.index] = 1812433253_u32
                .wrapping_mul(self.state[self.index - 1] ^ (self.state[self.index - 1] >> 30))
                + self.index as u32;
            self.state[self.index] &= 0xffffffff;

            self.index += 1;
        }
    }

    pub fn genrand(&mut self) -> u32 {
        if self.index >= N {
            twist(&mut self.state);

            self.index = 0;
        }

        let y = self.state[self.index];
        self.index += 1;

        temper(y)
    }

    pub fn load_state(&mut self, state: &[u32; 624], index: usize) {
        self.state = *state;
        self.index = index;
    }
}

impl Default for MT19937 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
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
}
