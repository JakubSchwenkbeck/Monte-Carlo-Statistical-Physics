use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution as RandDistribution;
use rand::Rng;

pub trait Distribution<T> {
    fn sample(&self, current_state: &T) -> T;
}

pub struct UniformDistribution {
    pub lower: f64,
    pub upper: f64,
}

impl Distribution<f64> for UniformDistribution {
    fn sample(&self, _current_state: &f64) -> f64 {
        let random_value = rand::random::<f64>();
        self.lower + random_value * (self.upper - self.lower)
    }
}

pub struct ProbabilityDistribution {
    pub values: Vec<f64>,
}

impl ProbabilityDistribution {
    pub fn new(values: Vec<f64>) -> Self {
        assert!(
            values.iter().all(|&x| x >= 0.0),
            "All values must be non-negative"
        );
        let sum: f64 = values.iter().sum();
        assert!((sum - 1.0).abs() < 1e-9, "Probabilities must sum to 1");
        Self { values }
    }
    // Add direct sampling capability to ProbabilityDistribution
    pub fn sample<R: Rng>(&self, rng: &mut R) -> usize {
        let dist = WeightedIndex::new(&self.values).expect("Invalid probability distribution");
        dist.sample(rng)
    }
}

pub struct RandomVariable<T> {
    pub state_space: Vec<T>,
    pub distribution: ProbabilityDistribution,
}

impl<T: Clone> RandomVariable<T> {
    pub fn sample<R: Rng>(&self, rng: &mut R) -> T {
        let dist = WeightedIndex::new(&self.distribution.values).unwrap();
        let index = dist.sample(rng);
        self.state_space[index].clone()
    }
}
