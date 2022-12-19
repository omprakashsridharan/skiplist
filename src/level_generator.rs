use core::panicking::panic;
use rand::prelude::*;

pub trait LevelGenerator {
    fn total(&self) -> usize;
    fn random(&mut self) -> usize;
}

pub struct GeometricalLevelGenerator {
    total: usize,
    p: f64,
    rng: SmallRng
}

impl GeometricalLevelGenerator {
    pub fn new(total: usize, p: f64) -> Self {
        if total == 0 {
            panic!("total must be non-zero");
        }
        if p <= 0.0 || p >= 1.0 {
            panic!("p must be in (0, 1).")
        }
        GeometricalLevelGenerator {
            total,
            p,
            rng: SmallRng::from_rng(thread_rng()).unwrap()
        }
    }
}

impl LevelGenerator for GeometricalLevelGenerator {
    fn total(&self) -> usize {
        self.total
    }

    fn random(&mut self) -> usize {
        let mut h = 0;
        let mut x = self.p;
        let f = 1.0 - self.rng.gen::<f64>();
        while x > f && h + 1 < self.total {
            h += 1;
            x *= self.p
        }
        h
    }
}