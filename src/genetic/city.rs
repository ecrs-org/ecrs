use rand::distributions::{Distribution, Standard};
use rand::{Rng, thread_rng};
use std::fmt::{self, Formatter};

#[derive(Copy, Clone)]
pub struct City {
    pub x:i32,
    pub y:i32,
}

impl Distribution<City> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> City {
        let randx = rng.gen_range(0..100);
        let randy = rng.gen_range(0..100);
        City {
            x:randx,
            y:randy,
        }
    }
}

impl PartialEq for City {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "X:{} Y:{}", self.x, self.y)
    }
}
