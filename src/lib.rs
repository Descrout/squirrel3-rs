pub mod sq3 {
    use std::time::Instant;
    use std::cell::Cell;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::thread;

    thread_local! {
        static RNG: Rng = Rng::new({
            let mut hasher = DefaultHasher::new();
            Instant::now().hash(&mut hasher);
            thread::current().id().hash(&mut hasher);
            let hash = hasher.finish();
            ((hash << 1) | 1) as u32
        });
    }

    pub struct Rng {
        position: Cell<u32>,
        _seed: Cell<u32>
    }

    impl Rng {
        pub fn new(seed: u32) -> Self {
            Self{
                position: Cell::new(0),
                _seed: Cell::new(seed)
            }
        }

        #[inline]
        pub fn seed(&self, value: u32) {
            self._seed.set(value);
        }

        #[inline]
        pub fn rand(&self) -> f32 {
            let new_pos = self.position.get() + 1;
            self.position.set(new_pos);
            normalized(new_pos, self._seed.get())
        }

        #[inline]
        pub fn bool(&self) -> bool {
            self.rand() < 0.5
        }

        #[inline]
        pub fn range_i32(&self, min: i32, max: i32) -> i32 {
            (self.rand() * (max as f32 - min as f32)) as i32 + min
        }

        #[inline]
        pub fn range_f32(&self, min: f32, max: f32) -> f32 {
            (self.rand() * (max - min)) + min
        }

        #[inline]
        pub fn range_u8(&self, min: u8, max: u8) -> u8 {
            (self.rand() * (max as f32 - min as f32)) as u8 + min
        }

        #[inline]
        pub fn range_usize(&self, min: usize, max: usize) -> usize {
            (self.rand() * (max as f32 - min as f32)) as usize + min
        }

        #[inline]
        pub fn range_u32(&self, min: u32, max: u32) -> u32 {
            (self.rand() * (max as f32 - min as f32)) as u32 + min
        }

        #[inline]
        pub fn u8(&self, max: u8) -> u8 {
            (self.rand() * max as f32) as u8
        }

        #[inline]
        pub fn usize(&self, max: usize) -> usize {
            (self.rand() * max as f32) as usize
        }

        #[inline]
        pub fn u32(&self, max: u32) -> u32 {
            (self.rand() * max as f32) as u32
        }

        #[inline]
        pub fn shuffle<T>(&self, slice: &mut [T]) {
            for n in 1..slice.len() {
                slice.swap(n, self.usize(n + 1));
            }
        }
    }

    // HELPER FUNCTIONS
    #[inline]
    pub fn rand() -> f32 {
        RNG.with(|rng| rng.rand())
    }

    #[inline]
    pub fn bool() -> bool {
        RNG.with(|rng| rng.bool())
    }

    #[inline]
    pub fn range_i32(min: i32, max: i32) -> i32 {
        RNG.with(|rng| rng.range_i32(min, max))
    }

    #[inline]
    pub fn range_f32(min: f32, max: f32) -> f32 {
        RNG.with(|rng| rng.range_f32(min, max))
    }

    #[inline]
    pub fn range_u8(min: u8, max: u8) -> u8 {
        RNG.with(|rng| rng.range_u8(min, max))
    }

    #[inline]
    pub fn range_usize(min: usize, max: usize) -> usize {
        RNG.with(|rng| rng.range_usize(min, max))
    }

    #[inline]
    pub fn range_u32(min: u32, max: u32) -> u32 {
        RNG.with(|rng| rng.range_u32(min, max))
    }

    #[inline]
    pub fn u8(max: u8) -> u8 {
        RNG.with(|rng| rng.u8(max))
    }

    #[inline]
    pub fn usize(max: usize) -> usize {
        RNG.with(|rng| rng.usize(max))
    }

    #[inline]
    pub fn u32(max: u32) -> u32 {
        RNG.with(|rng| rng.u32(max))
    }

    #[inline]
    pub fn seed(seed: u32) {
        RNG.with(|rng| rng.seed(seed))
    }

    #[inline]
    pub fn shuffle<T>(slice: &mut [T]) {
        RNG.with(|rng| rng.shuffle(slice))
    }

    // NO SIDE EFFECTS FUNCTIONS

    // https://www.youtube.com/watch?v=LWFzPP8ZbdU
    pub fn squirrel3(position: u32, seed: u32) -> u32 {
        let mut mangled = position.wrapping_mul(0xB5297A4D);
        mangled = mangled.wrapping_add(seed);
        mangled = mangled ^ (mangled >> 8);
        mangled = mangled.wrapping_add(0x68E31DA4);
        mangled = mangled ^ (mangled << 8);
        mangled = mangled.wrapping_mul(0x1B56C4E9);
        mangled = mangled ^ (mangled >> 8);
        mangled
    }

    #[inline]
    pub fn normalized(position: u32, seed: u32) -> f32 {
        squirrel3(position, seed) as f32 / u32::MAX as f32
    }

    #[inline]
    pub fn noise1d(position: u32) -> f32 {
        normalized(position, 0)
    }

    #[inline]
    pub fn noise2d(x: u32, y: u32) -> f32 {
        normalized(x.wrapping_add(y.wrapping_mul(198491317)), 0)
    }

    #[inline]
    pub fn noise3d(x: u32, y: u32, z: u32) -> f32 {
        normalized(x.wrapping_add(y.wrapping_mul(198491317)).wrapping_add(z.wrapping_mul(6542989)), 0)
    }
}