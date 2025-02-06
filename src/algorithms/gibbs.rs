#![allow(warnings)]

use rand::Rng;

pub struct GibbsSampler {
    pub num_variables: usize,
    pub conditional_distributions: Vec<Box<dyn Fn(f64) -> f64>>,
}

impl GibbsSampler {
    pub fn new(
        num_variables: usize,
        conditional_distributions: Vec<Box<dyn Fn(f64) -> f64>>,
    ) -> Self {
        assert_eq!(num_variables, conditional_distributions.len());
        Self {
            num_variables,
            conditional_distributions,
        }
    }

    pub fn sample(&self, num_samples: usize, initial_state: Vec<f64>) -> Vec<Vec<f64>> {
        let mut samples = Vec::with_capacity(num_samples);
        let mut current_state = initial_state;

        for _ in 0..num_samples {
            for i in 0..self.num_variables {
                let conditional_dist = &self.conditional_distributions[i];
                current_state[i] = conditional_dist(current_state[i]);
            }
            samples.push(current_state.clone());
        }

        samples
    }
}

pub fn gibbs_example() {
    // Example: Bivariate Gaussian distribution
    let conditional_x = |y: f64| -> f64 {
        let mut rng = rand::rng();
        rng.random_range(-1.0..1.0) + 0.5 * y
    };

    let conditional_y = |x: f64| -> f64 {
        let mut rng = rand::rng();
        rng.random_range(-1.0..1.0) + 0.5 * x
    };

    let gibbs_sampler =
        GibbsSampler::new(2, vec![Box::new(conditional_x), Box::new(conditional_y)]);
    let initial_state = vec![0.0, 0.0];
    let samples = gibbs_sampler.sample(10_000, initial_state);

    // Compute mean and variance of the samples
    let mean_x: f64 = samples.iter().map(|s| s[0]).sum::<f64>() / samples.len() as f64;
    let mean_y: f64 = samples.iter().map(|s| s[1]).sum::<f64>() / samples.len() as f64;

    println!("Gibbs Sampling Example");
    println!("Mean X: {}", mean_x);
    println!("Mean Y: {}", mean_y);
}

#[test]
fn test_gibbs_sampler() {
    let conditional_x = |y: f64| -> f64 {
        let mut rng = rand::rng();
        rng.random_range(-1.0..1.0) + 0.5 * y
    };

    let conditional_y = |x: f64| -> f64 {
        let mut rng = rand::rng();
        rng.random_range(-1.0..1.0) + 0.5 * x
    };

    let gibbs_sampler =
        GibbsSampler::new(2, vec![Box::new(conditional_x), Box::new(conditional_y)]);
    let initial_state = vec![0.0, 0.0];
    let samples = gibbs_sampler.sample(10_000, initial_state);

    // Compute mean and variance of the samples
    let mean_x: f64 = samples.iter().map(|s| s[0]).sum::<f64>() / samples.len() as f64;
    let mean_y: f64 = samples.iter().map(|s| s[1]).sum::<f64>() / samples.len() as f64;

    // Check that the means are close to 0
    assert!((mean_x.abs() - 0.0).abs() < 0.1, "Mean X is not close to 0");
    assert!((mean_y.abs() - 0.0).abs() < 0.1, "Mean Y is not close to 0");
}
