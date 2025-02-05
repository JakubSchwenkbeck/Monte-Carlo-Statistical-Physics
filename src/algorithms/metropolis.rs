use rand::Rng;

/// Metropolis algorithm for sampling from a target distribution.
pub struct Metropolis {
    target_distribution: fn(f64) -> f64, // Target distribution (up to a normalizing constant)
    step_size: f64,                      // Step size for proposing new states
}

impl Metropolis {
    /// Create a new Metropolis sampler.
    pub fn new(target_distribution: fn(f64) -> f64, step_size: f64) -> Self {
        Self {
            target_distribution,
            step_size,
        }
    }

    /// Generate samples from the target distribution.
    pub fn sample(&self, num_samples: usize, initial_state: f64) -> Vec<f64> {
        let mut rng = rand::rng();
        let mut samples = Vec::with_capacity(num_samples);
        let mut current_state = initial_state;

        for _ in 0..num_samples {
            // Propose a new state
            let proposed_state = current_state + rng.random_range(-self.step_size..self.step_size);

            // Compute acceptance probability
            let acceptance_prob = ((self.target_distribution)(proposed_state)
                / (self.target_distribution)(current_state))
            .min(1.0);

            // Accept or reject the new state
            if rng.random::<f64>() < acceptance_prob {
                current_state = proposed_state;
            }

            // Save the current state as a sample
            samples.push(current_state);
        }

        samples
    }
}

pub fn metropolis_example() {
    // Target distribution: Standard Gaussian (up to a normalizing constant)
    fn target_distribution(x: f64) -> f64 {
        (-x * x / 2.0).exp()
    }

    // Create a Metropolis sampler
    let metropolis = Metropolis::new(target_distribution, 1.0);

    // Generate samples
    let num_samples = 10_000;
    let initial_state = 0.0;
    let samples = metropolis.sample(num_samples, initial_state);

    // Compute mean and variance of the samples
    let mean: f64 = samples.iter().sum::<f64>() / num_samples as f64;
    let variance: f64 =
        samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / num_samples as f64;

    // Print results
    println!("Metropolis Algorithm Example");
    println!("Target Distribution: Standard Gaussian");
    println!("Number of Samples: {}", num_samples);
    println!("Mean: {}", mean);
    println!("Variance: {}", variance);
}
