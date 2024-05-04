use crate::ga::Metrics;

use super::ValueProvider;

pub struct FlatValue<T: Sized + Clone> {
    value: T,
}

impl<T: Sized + Clone> ValueProvider<T> for FlatValue<T> {
    #[inline]
    fn get(&mut self, _metrics: &Metrics) -> T {
        self.value.clone()
    }
}

// WE MUST ADD PLATFORM CHECKS HERE, TO VERIFY WHETHER GIVEN TYPES
// ARE AVAILABLE ON GIVEN PLATFORM...

impl ValueProvider<usize> for usize {
    fn get(&mut self, _metrics: &Metrics) -> usize {
        *self
    }
}

impl ValueProvider<isize> for isize {
    fn get(&mut self, _metrics: &Metrics) -> isize {
        *self
    }
}

impl ValueProvider<i32> for i32 {
    fn get(&mut self, _metrics: &Metrics) -> i32 {
        *self
    }
}

impl ValueProvider<i64> for i64 {
    fn get(&mut self, _metrics: &Metrics) -> i64 {
        *self
    }
}

impl ValueProvider<i8> for i8 {
    fn get(&mut self, _metrics: &Metrics) -> i8 {
        *self
    }
}

impl ValueProvider<u8> for u8 {
    fn get(&mut self, _metrics: &Metrics) -> u8 {
        *self
    }
}

impl ValueProvider<f32> for f32 {
    fn get(&mut self, _metrics: &Metrics) -> f32 {
        *self
    }
}

impl ValueProvider<f64> for f64 {
    fn get(&mut self, _metrics: &Metrics) -> f64 {
        *self
    }
}
