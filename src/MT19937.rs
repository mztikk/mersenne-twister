pub const DEFAULT_SEED: u32 = 5489;
pub const DEFAULT_SEED_PS2: u32 = 4537;

const N: usize = 624;

const MATRIX_A: u32 = 0x9908b0df;
const UPPER_MASK: u32 = 0x80000000;
const LOWER_MASK: u32 = 0x7fffffff;

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

    fn twist(&mut self) {
        for i in 0..N {
            let x = (self.state[i] & UPPER_MASK) + (self.state[(i + 1) % N] & LOWER_MASK);
            let mut x_a = x >> 1;

            if x % 2 != 0 {
                x_a ^= MATRIX_A;
            }

            self.state[i] = self.state[(i + 397) % N] ^ x_a;
        }

        self.index = 0;
    }

    fn temper(&self, y: u32) -> u32 {
        let mut y = y;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;

        y
    }

    pub fn genrand(&mut self) -> u32 {
        if self.index >= N {
            self.twist();
        }

        let y = self.state[self.index];
        self.index += 1;

        self.temper(y)
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
