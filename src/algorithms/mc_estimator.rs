use crate::algorithms::distributions::ProbabilityDistribution;
use crate::algorithms::stochastic_matrix::StochasticMatrix;

pub struct MonteCarloEstimator {
    pub transition_matrix: StochasticMatrix,
}

impl MonteCarloEstimator {
    pub fn new(transition_matrix: StochasticMatrix) -> Self {
        Self { transition_matrix }
    }

    pub fn estimate(
        &self,
        initial_distribution: &ProbabilityDistribution,
        n_steps: usize,
        n_simulations: usize,
    ) -> ProbabilityDistribution {
        let mut counts = vec![0; initial_distribution.values.len()];
        let mut rng = rand::rng();

        for _ in 0..n_simulations {
            let mut current_state = initial_distribution.sample(&mut rng);

            for _ in 0..n_steps {
                let row = &self.transition_matrix.matrix[current_state];
                let row_dist = ProbabilityDistribution::new(row.clone());
                current_state = row_dist.sample(&mut rng);
            }

            counts[current_state] += 1;
        }

        let total = n_simulations as f64;
        let values = counts.into_iter().map(|c| c as f64 / total).collect();

        ProbabilityDistribution::new(values)
    }
    // This method estimates a weighted average for a general observable f.
    /// f: a function mapping a state index (usize) to an observable value (f64)
    /// pi: the stationary probability distribution (as a vector) for the states.
    pub fn estimate_weighted<F>(
        &self,
        initial_distribution: &ProbabilityDistribution,
        n_steps: usize,
        n_simulations: usize,
        f: F,
        pi: &[f64],
    ) -> f64
    where
        F: Fn(usize) -> f64,
    {
        // numerator and denominator accumulators.
        let mut weighted_sum = 0.0;
        let mut weight_sum = 0.0;
        let mut rng = rand::rng();

        for _ in 0..n_simulations {
            let mut current_state = initial_distribution.sample(&mut rng);
            for _ in 0..n_steps {
                let row = &self.transition_matrix.matrix[current_state];
                let row_dist = ProbabilityDistribution::new(row.clone());
                current_state = row_dist.sample(&mut rng);
            }
            // weight for the current state is 1/pi[current_state]
            let w = 1.0 / pi[current_state];
            weighted_sum += f(current_state) * w;
            weight_sum += w;
        }

        weighted_sum / weight_sum
    }

    /// Estimate the canonical expectation value of the Hamiltonian.
    /// H: a function that computes the energy for a given state.
    pub fn estimate_energy<F>(
        &self,
        initial_distribution: &ProbabilityDistribution,
        n_steps: usize,
        n_simulations: usize,
        h: F,
        beta: f64,
        pi: &[f64],
    ) -> f64
    where
        F: Fn(usize) -> f64,
    {
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        let mut rng = rand::rng();

        for _ in 0..n_simulations {
            let mut current_state = initial_distribution.sample(&mut rng);
            for _ in 0..n_steps {
                let row = &self.transition_matrix.matrix[current_state];
                let row_dist = ProbabilityDistribution::new(row.clone());
                current_state = row_dist.sample(&mut rng);
            }
            let energy = h(current_state);
            // reweight factor is e^(-beta H) / pi[state]
            let weight = (-beta * energy).exp() / pi[current_state];
            numerator += energy * weight;
            denominator += weight;
        }
        numerator / denominator
    }
}
