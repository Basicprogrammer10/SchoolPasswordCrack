pub struct Random {
    seed: i128,
    a: i128,
    c: i128,
    m: i128,

    start_m: i128,
}

impl Random {
    pub fn new(seed: i128) -> Random {
        Random {
            seed,
            a: 16_807,
            c: 0,
            m: 2_147_483_647,
            start_m: 2_147_483_647,
        }
    }

    pub fn next_f64(&mut self) -> f64 {
        self.seed = (self.a * self.seed + self.c) % self.m;
        self.seed as f64 / self.start_m as f64
    }

    pub fn next_int_i64(&mut self, min: i64, max: i64) -> i64 {
        let x = self.next_f64();
        (x * (max - min + 1) as f64 + min as f64) as i64
    }
}
