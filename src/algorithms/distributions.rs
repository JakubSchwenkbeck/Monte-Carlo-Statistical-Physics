/// A trait representing a generic probability distribution.
/// It defines a function to sample a new value given the current state.
// (may not need the current state?)
pub trait Distribution<T> {
    fn sample(&self, current_state: &T) -> T;
}

/// implement a simple uniform distribution
pub struct UniformDistribution {
    pub lower: f64,
    pub upper: f64,
}
impl Distribution<f64> for UniformDistribution {
    fn sample(&self, _current_state: &f64) -> f64 {
        let random_value = rand::random::<f64>(); // using the rand crate for now
        self.lower + random_value * (self.upper - self.lower)
    }
}
