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
}
